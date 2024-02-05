package main

import (
	"machine"

	"tinygo.org/x/drivers/lis3dh"
	"tinygo.org/x/drivers/shifter"
	"tinygo.org/x/drivers/st7735"
	"tinygo.org/x/drivers/ws2812"
)

type Device struct {
	Display *st7735.Device
	Buttons *shifter.Device
	LEDs    *ws2812.Device
	Accel   *lis3dh.Device
	Buzzer  *machine.Pin
	Speaker *machine.Pin
}

func NewDevice() Device {
	machine.SPI1.Configure(machine.SPIConfig{
		SCK:       machine.SPI1_SCK_PIN,
		SDO:       machine.SPI1_SDO_PIN,
		SDI:       machine.SPI1_SDI_PIN,
		Frequency: 8000000,
	})
	machine.I2C0.Configure(machine.I2CConfig{SCL: machine.SCL_PIN, SDA: machine.SDA_PIN})

	accel := lis3dh.New(machine.I2C0)
	accel.Address = lis3dh.Address0
	accel.Configure()

	display := st7735.New(machine.SPI1, machine.TFT_RST, machine.TFT_DC, machine.TFT_CS, machine.TFT_LITE)
	display.Configure(st7735.Config{
		Rotation: st7735.ROTATION_90,
	})

	buttons := shifter.NewButtons()
	buttons.Configure()

	neo := machine.NEOPIXELS
	neo.Configure(machine.PinConfig{Mode: machine.PinOutput})
	leds := ws2812.New(neo)

	bzrPin := machine.A0
	bzrPin.Configure(machine.PinConfig{Mode: machine.PinOutput})

	speaker := machine.SPEAKER_ENABLE
	speaker.Configure(machine.PinConfig{Mode: machine.PinOutput})
	speaker.High()

	return Device{
		Display: &display,
		Buttons: &buttons,
		LEDs:    &leds,
		Buzzer:  &bzrPin,
		Accel:   &accel,
		Speaker: &speaker,
	}
}

func (d Device) ReadButtons() (Buttons, error) {
	bs, err := d.Buttons.Read8Input()
	return Buttons{bs}, err
}
