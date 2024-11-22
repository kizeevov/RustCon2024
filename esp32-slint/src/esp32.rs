use crate::instant;
use alloc::{boxed::Box, rc::Rc};
use button_driver::{Button, ButtonConfig, Mode};
use core::{cell::RefCell, convert::Infallible, mem::MaybeUninit, time::Duration};
use display_interface_spi::SPIInterface;
use embedded_graphics_core::geometry::OriginDimensions;
use embedded_hal::digital::OutputPin;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Io, Level, Output, Pull},
    prelude::*,
    rtc_cntl::Rtc,
    spi::master::Spi,
    spi::SpiMode,
    time::now,
    timer::timg::TimerGroup,
};
use log::info;
use mipidsi::options::ColorInversion;
use slint::{
    platform::{Key, WindowAdapter, WindowEvent},
    PlatformError,
};

type Display<DI, RST> = mipidsi::Display<DI, mipidsi::models::ST7789, RST>;

pub fn init() {
    const HEAP_SIZE: usize = 160 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();
    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            HEAP.as_mut_ptr() as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
    slint::platform::set_platform(Box::new(EspBackend::default()))
        .expect("backend already initialized");
}

#[derive(Default)]
struct EspBackend {
    window: RefCell<Option<Rc<slint::platform::software_renderer::MinimalSoftwareWindow>>>,
}

impl slint::platform::Platform for EspBackend {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        let window = slint::platform::software_renderer::MinimalSoftwareWindow::new(
            slint::platform::software_renderer::RepaintBufferType::ReusedBuffer,
        );
        self.window.replace(Some(window.clone()));
        Ok(window)
    }

    fn duration_since_start(&self) -> Duration {
        Duration::from_micros(now().ticks())
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        let peripherals = esp_hal::init(esp_hal::Config::default());

        // Disable the RTC and TIMG watchdog timers
        let mut rtc = Rtc::new(peripherals.LPWR);
        rtc.rwdt.disable();
        let mut timer_group0 = TimerGroup::new(peripherals.TIMG0);
        timer_group0.wdt.disable();
        let mut timer_group1 = TimerGroup::new(peripherals.TIMG1);
        timer_group1.wdt.disable();

        let mut delay = Delay::new();
        let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

        let mut backlight = Output::new(io.pins.gpio4, Level::High);
        backlight.set_high();

        let mosi = io.pins.gpio19;
        let cs = Output::new(io.pins.gpio5, Level::Low);
        let rst = Output::new(io.pins.gpio23, Level::Low);
        let dc = io.pins.gpio16;
        let sck = io.pins.gpio18;

        let spi = Spi::new(peripherals.SPI2, 60u32.MHz(), SpiMode::Mode0).with_pins(
            sck,
            mosi,
            esp_hal::gpio::NoPin,
            esp_hal::gpio::NoPin,
        );
        let spi = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap();
        let di = SPIInterface::new(spi, Output::new(dc, Level::Low));

        let display = mipidsi::Builder::new(mipidsi::models::ST7789, di)
            .reset_pin(rst)
            .display_offset(52, 40)
            .display_size(135, 240)
            .invert_colors(ColorInversion::Inverted)
            .init(&mut delay)
            .unwrap();

        let size = display.size();
        let size = slint::PhysicalSize::new(size.width, size.height);

        let mut buffer_provider = DrawBuffer {
            display,
            buffer: &mut [slint::platform::software_renderer::Rgb565Pixel(0); 135],
        };

        self.window.borrow().as_ref().unwrap().set_size(size);

        let up_button_pin = Input::new(io.pins.gpio35, Pull::Up);
        let down_button_pin = Input::new(io.pins.gpio0, Pull::Up);
        let button_config = ButtonConfig {
            mode: Mode::PullUp,
            ..ButtonConfig::default()
        };

        let mut up_button = Button::<_, instant::Instant>::new(up_button_pin, button_config);
        let mut down_button = Button::<_, instant::Instant>::new(down_button_pin, button_config);

        loop {
            slint::platform::update_timers_and_animations();
            up_button.tick();
            down_button.tick();

            if let Some(window) = self.window.borrow().clone() {
                if up_button.is_clicked() {
                    window.dispatch_event(WindowEvent::KeyPressed {
                        text: Key::UpArrow.into(),
                    });
                } else if up_button.holds() == 1 {
                    window.dispatch_event(WindowEvent::KeyPressed {
                        text: Key::RightArrow.into(),
                    });
                }

                if down_button.is_clicked() {
                    window.dispatch_event(WindowEvent::KeyPressed {
                        text: Key::DownArrow.into(),
                    });
                } else if down_button.holds() == 1 {
                    info!("Held");
                    window.dispatch_event(WindowEvent::KeyPressed {
                        text: Key::LeftArrow.into(),
                    });
                }

                window.draw_if_needed(|renderer| {
                    renderer.render_by_line(&mut buffer_provider);
                });

                // TODO
                // if window.has_active_animations() {
                //     continue;
                // }
            }

            up_button.reset();
            down_button.reset();
        }
    }
}

struct DrawBuffer<'a, Display> {
    display: Display,
    buffer: &'a mut [slint::platform::software_renderer::Rgb565Pixel],
}

impl<DI: display_interface::WriteOnlyDataCommand, RST: OutputPin<Error = Infallible>>
    slint::platform::software_renderer::LineBufferProvider
    for &mut DrawBuffer<'_, Display<DI, RST>>
{
    type TargetPixel = slint::platform::software_renderer::Rgb565Pixel;

    fn process_line(
        &mut self,
        line: usize,
        range: core::ops::Range<usize>,
        render_fn: impl FnOnce(&mut [slint::platform::software_renderer::Rgb565Pixel]),
    ) {
        let buffer = &mut self.buffer[range.clone()];
        render_fn(buffer);
        self.display
            .set_pixels(
                range.start as u16,
                line as u16,
                range.end as u16,
                line as u16,
                buffer
                    .iter()
                    .map(|x| embedded_graphics_core::pixelcolor::raw::RawU16::new(x.0).into()),
            )
            .unwrap();
    }
}
