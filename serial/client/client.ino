// client.ino
// -*- mode: C++ -*-

#include <SPI.h>
#include <AutomatoMsg.h>
#include <pins_arduino.h>
#include <Automato.h>

// ideally this would go in a shared header file,
struct ServerData {
  char name[25];
  float targettemp;
  uint64_t macAddress;
  float temperature;
  float humidity;
};

ServerData serverdata;

MapField memoryMap[] = 
  { map_field(ServerData, name, ff_char)
  , map_field(ServerData, targettemp, ff_float)
  }; 


Automato automato(1, (void*)&serverdata, sizeof(ServerData), (void*)&memoryMap, 2, true);

// the automato we're going to control remotely.
uint8_t serveraddr(2);

bool on;

uint8_t loops;

void setup()
{
  pinMode(PIN_LORA_RST, INPUT); // Let the pin float.

  // Disable other SPI devices.
  pinMode(PIN_LCD_CS, OUTPUT);
  digitalWrite(PIN_LCD_CS, HIGH);
  pinMode(PIN_TCH_CS, OUTPUT);
  digitalWrite(PIN_TCH_CS, HIGH);
  pinMode(PIN_SD_CS, OUTPUT);
  digitalWrite(PIN_SD_CS, HIGH);

  Serial.begin(115200);

  // Serial.println("automato remote control client");

  automato.init(915.0, 20);

  // print my mac id.
  // Serial.print("my mac address:");
  // Serial.println(Automato::macAddress());

  on = true;

  pinMode(A0, INPUT);
  pinMode(A1, INPUT);
  pinMode(A6, INPUT);
  pinMode(A7, INPUT);

  loops = 0;
}

void loop()
{
  automato.doSerial();
}
