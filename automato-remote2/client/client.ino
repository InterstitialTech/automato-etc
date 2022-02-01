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

// the automato we're going to control remotely.
Automato automato(1, NULL, 0);

uint8_t serveraddr(2);

bool on;

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

  Serial.println("automato remote control client");

  automato.init();

  // print my mac id.
  Serial.print("my mac address:");
  Serial.println(Automato::macAddress());

  on = true;

  pinMode(A0, INPUT);
  pinMode(A1, INPUT);
  pinMode(A6, INPUT);
  pinMode(A7, INPUT);

}

void loop()
{
  // write to a pin on the remote automato.
  if (automato.remoteDigitalWrite(serveraddr, PIN_LED, (on ? 1 : 0))) 
  {
    Serial.print("successful write: ");
    Serial.println(on);
    on = !on;
  }
  else 
  {
    Serial.println("write failed!");
  }

  // read a char field from the remote.
  char remotename[sizeof(ServerData::name)];
  if (automato.remote_memread(serveraddr,
                             ServerData,
                             name,
                             remotename))
  {
    Serial.print("retrieved remote name: ");
    Serial.println(remotename);
  }
  else 
  {
    Serial.println("failed to retrieve remote name!");
  }

  float temp = 75;
  if (automato.remote_memwrite(serveraddr,
                               ServerData,
                               targettemp,
                               &temp))
  {
    Serial.print("wrote remote temp: ");
    Serial.println(temp);
  }
  else 
  {
    Serial.println("failed to write remote temp!");
  }

  // read digital status of a pin on the remote.
  uint8_t a7;
  automato.remotePinMode(serveraddr, A7, INPUT);
  if (automato.remoteDigitalRead(serveraddr, A7, &a7)) 
  {
    Serial.print("read remote pin: ");
    Serial.println(a7);

    digitalWrite(PIN_LED, a7);
  }
  else 
  {
    Serial.println("read remote pin failed!");
  }

  float temperature;
  float humidity;
  if (automato.remoteTemperature(serveraddr, temperature))
  {
    Serial.print("remote temperature: ");
    Serial.println(temperature);
  }
  else
  {
    Serial.println("error retrieving temperature");
  }
  if (automato.remoteHumidity(serveraddr, humidity))
  {
    Serial.print("remote humidity: ");
    Serial.println(humidity);
  }
  else
  {
    Serial.println("error retrieving humidity");
  }

  RemoteInfo serverinfo;
  if (automato.remoteAutomatoInfo(serveraddr, serverinfo)) 
  {
    Serial.println("remote server info:");
    Serial.print("protoversion: ");
    Serial.println(serverinfo.protoversion);
    Serial.print("macAddress: ");
    Serial.println(serverinfo.macAddress);
    Serial.print("datalen: ");
    Serial.println(serverinfo.datalen);
  }
  else
  {
    Serial.print("failed to retrieve remote info!");
  }
}
