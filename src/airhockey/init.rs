///! Init airhockey game
use crate::i2c::I2C;
use crate::{
    gpio::{GpioPort, OutputPin},
    init, lcd,
    system_clock::Hz,
};
use stm32f7::stm32f7x6::I2C3;
use stm32f7::stm32f7x6::{CorePeripherals, Peripherals};

use crate::{graphics::graphics::Graphics, input::input::Input, physics::physics::Physics};

use super::{
    ball, ball::Ball, field, game, game::Game, graphics_handler, graphics_handler::GraphicsHandler,
    input_handler, input_handler::InputHandler, physics_handler, physics_handler::PhysicsHandler,
    player, player::Player, score, score::Score,
};

/// Handler object to access hardware components
pub struct Handler {
    /// struct field
    pub physics_handler: PhysicsHandler,
    /// struct field
    pub graphics_handler: GraphicsHandler,
    /// struct field
    pub input_handler: InputHandler,
}

impl Handler {
    /// handler constructor
    pub fn new(
        physics_handler: PhysicsHandler,
        graphics_handler: GraphicsHandler,
        input_handler: InputHandler,
    ) -> Handler {
        Handler {
            physics_handler: physics_handler,
            graphics_handler: graphics_handler,
            input_handler: input_handler,
        }
    }
}

/// Function init
pub fn init(player_count: u8, handler: Handler) -> Game {
    let game = Game::new(player_count, handler);
    return game;
}
/// create handler function
pub fn create_handler() -> Handler {
    let fps: u16 = 30;
    let hardware: (
        (
            lcd::Layer<lcd::FramebufferArgb8888>,
            lcd::Layer<lcd::FramebufferAl88>,
        ),
        I2C<I2C3>,
    ) = init_general_hardware(fps);
    let layers = ((hardware.0).0, (hardware.0).1);
    let graphics = Graphics::new(field::WIDTH_MAX, field::HEIGHT_MAX, layers);
    let input = Input::new(field::WIDTH_MAX, field::HEIGHT_MAX, hardware.1);
    let physics = Physics::new(field::WIDTH_MAX, field::HEIGHT_MAX);

    return Handler::new(
        PhysicsHandler::new(physics),
        GraphicsHandler::new(graphics),
        InputHandler::new(input),
    );
}

/// init the general hardware
pub fn init_general_hardware(
    fps: u16,
) -> (
    (
        lcd::Layer<lcd::FramebufferArgb8888>,
        lcd::Layer<lcd::FramebufferAl88>,
    ),
    I2C<I2C3>,
) {
    /// initialising LCD screen
    let core_peripherals = CorePeripherals::take().unwrap();
    let mut systick = core_peripherals.SYST;
    let nvic = core_peripherals.NVIC;

    let peripherals = Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC;
    let pwr = peripherals.PWR;
    let flash = peripherals.FLASH;
    let mut fmc = peripherals.FMC;
    let mut ltdc = peripherals.LTDC;
    let sai_2 = peripherals.SAI2;
    let rng = peripherals.RNG;
    let sdmmc = peripherals.SDMMC1;
    let syscfg = peripherals.SYSCFG;
    let ethernet_mac = peripherals.ETHERNET_MAC;
    let ethernet_dma = peripherals.ETHERNET_DMA;

    init::init_systick(Hz(100), &mut systick, &rcc);
    systick.enable_interrupt();

    /// Initialise display port
    let gpio_a = GpioPort::new(peripherals.GPIOA);
    let gpio_b = GpioPort::new(peripherals.GPIOB);
    let gpio_c = GpioPort::new(peripherals.GPIOC);
    let gpio_d = GpioPort::new(peripherals.GPIOD);
    let gpio_e = GpioPort::new(peripherals.GPIOE);
    let gpio_f = GpioPort::new(peripherals.GPIOF);
    let gpio_g = GpioPort::new(peripherals.GPIOG);
    let gpio_h = GpioPort::new(peripherals.GPIOH);
    let gpio_i = GpioPort::new(peripherals.GPIOI);
    let gpio_j = GpioPort::new(peripherals.GPIOJ);
    let gpio_k = GpioPort::new(peripherals.GPIOK);
    let mut pins = init::pins(
        gpio_a, gpio_b, gpio_c, gpio_d, gpio_e, gpio_f, gpio_g, gpio_h, gpio_i, gpio_j, gpio_k,
    );

    init::init_sdram(&mut rcc, &mut fmc);
    let mut lcd = init::init_lcd(&mut ltdc, &mut rcc);
    pins.display_enable.set(true);
    pins.backlight.set(true);

    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();

    layer_1.clear();
    layer_2.clear();
    let lcd_display: (
        lcd::Layer<lcd::FramebufferArgb8888>,
        lcd::Layer<lcd::FramebufferAl88>,
    ) = (layer_1, layer_2);

    // Initialisig touch
    let i2c_3 = init::init_i2c_3(peripherals.I2C3, &mut rcc);
    return (lcd_display, i2c_3);
}
