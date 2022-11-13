// remotelora.ino
// -*- mode: C++ -*-

#include <SPI.h>
#include <RH_RF95.h>
#include <AutomatoMsg.h>
#include <Automato.h>

// ideally this would go in a shared header file,
struct ServerData {
  char name[25];
  float targettemp;
  int32_t loops;
  uint64_t macAddress;
  float temperature;
  float humidity;
};

ServerData serverdata;

MapField memoryMap[] = 
  { map_field(ServerData, name, ff_char)
  , map_field(ServerData, targettemp, ff_float)
  , map_field(ServerData, loops, ff_int32)
  }; 

Automato automato(2, (void*)&serverdata, sizeof(ServerData), (void*)&memoryMap, 3, true);

void setup()
{
  pinMode(PIN_LORA_RST, INPUT); // Let the pin float.
  pinMode(PIN_LED, OUTPUT);

  // Disable SPI devices until needed.
  pinMode(PIN_LCD_CS, OUTPUT);
  digitalWrite(PIN_LCD_CS, HIGH);
  pinMode(PIN_TCH_CS, OUTPUT);
  digitalWrite(PIN_TCH_CS, HIGH);
  pinMode(PIN_SD_CS, OUTPUT);
  digitalWrite(PIN_SD_CS, HIGH);

  Serial.begin(115200);

  automato.init(915.0, 20);

  Serial.println("automato remote control server");

  Serial.print("my mac address:");
  Serial.println(Automato::macAddress());


  strcpy(serverdata.name, "remotelora");
  serverdata.targettemp = 70;
  serverdata.macAddress = automato.macAddress();
  serverdata.temperature = automato.getTemperature();
  serverdata.humidity = automato.getHumidity();

  // pinMode(A0, INPUT);
  // pinMode(A1, INPUT);
  // pinMode(A6, INPUT);
  // pinMode(A7, INPUT);

  serverdata.loops = 0;

}


AutomatoResult ar;

void loop()
{
  if (!(ar = automato.doRemoteControl()))
  {
    Serial.println("-------- failure ---------");
    Serial.println("error from doRemoteControl():");
    Serial.println(ar.as_string());
    Serial.print("error code:");
    Serial.println(ar.resultCode());
  }

  serverdata.loops++;
}
