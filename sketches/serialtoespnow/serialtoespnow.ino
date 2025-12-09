// serialtolora.ino
// -*- mode: C++ -*-

#include <SPI.h>
#include <AutomatoMsg.h>
#include <pins_arduino.h>
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


Automato automato(1, (void*)&serverdata, sizeof(ServerData), (void*)&memoryMap, 3, true);

bool on;


void callback(const esp_now_recv_info *info, const uint8_t *data, int len) {

  // Serial.print("received");
  // write message to serial port.
  Serial.write('e');       // 'm' for message
  for (int i = 0; i<6; i++)
  {
      Serial.write(*(info->src_addr + i));   // from which automato.
  }
  uint8_t u8l = len;
  Serial.write(u8l);        // payload length
  Serial.write(data, len);
}


void setup()
{
  // pinMode(PIN_LORA_RST, INPUT); // Let the pin float.

  // Disable other SPI devices.
  // pinMode(PIN_LCD_CS, OUTPUT);
  // digitalWrite(PIN_LCD_CS, HIGH);
  // pinMode(PIN_TCH_CS, OUTPUT);
  // digitalWrite(PIN_TCH_CS, HIGH);
  // pinMode(PIN_SD_CS, OUTPUT);
  // digitalWrite(PIN_SD_CS, HIGH);

  Serial.begin(115200);

  automato.init();
  automato.initEspNow();
  automato.setCallbackEspNow(callback);

  on = true;
  serverdata.loops = 0;

  strcpy(serverdata.name, "test 1 2 3");
  serverdata.targettemp = 42.0;
 
}

void loop()
{
  automato.doSerial();

  // Serial.print("meh");
  // Serial.println(serverdata.loops);
  serverdata.loops++;
}
