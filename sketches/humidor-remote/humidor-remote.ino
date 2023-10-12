// remotelora.ino
// -*- mode: C++ -*-

#include <SPI.h>
#include <RH_RF95.h>
#include <AutomatoMsg.h>
#include <Automato.h>
#include <EEPROM.h>

#define EEPROM_SIZE 8



// ideally this would go in a shared header file,
struct ServerData {
  char name[25];
  float lowertargethumidity;
  float uppertargethumidity;
  uint32_t loops;
  uint32_t checkInterval;
};

ServerData serverdata;

MapField memoryMap[] =
  { map_field(ServerData, name, ff_char)
  , map_field(ServerData, lowertargethumidity, ff_float)
  , map_field(ServerData, uppertargethumidity, ff_float)
  , map_field(ServerData, loops, ff_uint32)
  , map_field(ServerData, checkInterval, ff_uint32)
  };

Automato automato(2, (void*)&serverdata, sizeof(ServerData), (void*)&memoryMap, sizeof(memoryMap) / sizeof(MapField), true);

int8_t output_pin = 33;

AutomatoResult ar;

int32_t lastCheck;

// keep these the same as what's in EEPROM.
bool lowertargethumidity_is_in_eep;
float lowertargethumidity_eep;
bool uppertargethumidity_is_in_eep;
float uppertargethumidity_eep;

void readFromFlash()
{
  bool all255 = true;
  for (int i = 0; i < 4; ++i)
  {
    *((unsigned char*)&lowertargethumidity_eep + i) = EEPROM.read(i);
    if (*((unsigned char*)&lowertargethumidity_eep + i) != 255)
      all255 = false;
  }
  lowertargethumidity_is_in_eep = !all255;

  for (int i = 0; i < 4; ++i)
  {
    *((unsigned char*)&uppertargethumidity_eep + i) = EEPROM.read(i + 4);
    if (*((unsigned char*)&uppertargethumidity_eep + i) != 255)
      all255 = false;
  }
  uppertargethumidity_is_in_eep = !all255;
}


float origLTH;
float origUTH;


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

  EEPROM.begin(EEPROM_SIZE);

  readFromFlash();

  origLTH = lowertargethumidity_eep;
  origUTH = uppertargethumidity_eep;


  automato.init(915.0, 20);

  Serial.println("automato remote control server");

  Serial.print("my mac address:");
  Serial.println(Automato::macAddress());

  // set serverdata vals.
  strcpy(serverdata.name, "humidor");

  // serverdata.lowertargethumidity = 35;
  // serverdata.uppertargethumidity = 39;

  // saveLTHIfChanged();
  // saveUTHIfChanged();

  serverdata.lowertargethumidity = lowertargethumidity_eep;
  serverdata.uppertargethumidity = uppertargethumidity_eep;
  if (lowertargethumidity_is_in_eep)
    serverdata.lowertargethumidity = lowertargethumidity_eep;
  else
    serverdata.lowertargethumidity = 45;

  if (uppertargethumidity_is_in_eep)
    serverdata.uppertargethumidity = uppertargethumidity_eep;
  else
    serverdata.uppertargethumidity = 49;

  serverdata.loops = 0;
  serverdata.checkInterval = 5000;

  lastCheck = 0;

  pinMode(output_pin, OUTPUT);

}


void printargets()
{
  Serial.print("origLTH ");
  Serial.println(origLTH);
  Serial.print("origUTH ");
  Serial.println(origUTH);
  Serial.print("lowertargethumidity_is_in_eep ");
  Serial.println(lowertargethumidity_is_in_eep);
  Serial.print("lowertargethumidity_eep ");
  Serial.println(lowertargethumidity_eep);
  Serial.print("uppertargethumidity_is_in_eep ");
  Serial.println(uppertargethumidity_is_in_eep);
  Serial.print("uppertargethumidity_eep ");
  Serial.println(uppertargethumidity_eep);
  Serial.print("serverdata LTH ");
  Serial.println(serverdata.lowertargethumidity);
  Serial.print("serverdata UTH ");
  Serial.println(serverdata.uppertargethumidity);
}

void saveLTHIfChanged()
{
  if (!lowertargethumidity_is_in_eep || (lowertargethumidity_eep != serverdata.lowertargethumidity))
  {
    Serial.println("saving LTH!");
    for (int i = 0; i < 4; ++i)
    {
      Serial.print("writing: ");
      Serial.print(i);
      Serial.print(" ");
      Serial.println(*((unsigned char*)(&serverdata.lowertargethumidity + i)));
      EEPROM.write(i, *((unsigned char*)(&serverdata.lowertargethumidity + i)));
    }
    lowertargethumidity_is_in_eep = true;
    lowertargethumidity_eep = serverdata.lowertargethumidity;
  }
  else
  {
    Serial.println("NOT saving LTH!");
  }
}

void saveUTHIfChanged()
{
  if (!uppertargethumidity_is_in_eep || (uppertargethumidity_eep != serverdata.uppertargethumidity))
  {
    Serial.println("saving UTH!");
    for (int i = 0; i < 4; ++i)
    {
      Serial.print("writing: ");
      Serial.print(i);
      Serial.print(" ");
      Serial.println(*((unsigned char*)(&serverdata.uppertargethumidity + i)));
      EEPROM.write(i + 4, *((unsigned char*)(&serverdata.uppertargethumidity + i)));
    }
    uppertargethumidity_is_in_eep = true;
    uppertargethumidity_eep = serverdata.uppertargethumidity;
  }
  else
  {
    Serial.println("NOT saving UTH!");
  }
}

// void loop() {
  // if lower/upper targets changed, save to eeprom.
  // saveLTHIfChanged();
// }

void loop()
{
  // automato.doSerial();

  printargets();

  if (!(ar = automato.doRemoteControl()))
  {
    Serial.println("-------- failure ---------");
    Serial.println("error from doRemoteControl():");
    Serial.println(ar.as_string());
    Serial.print("error code:");
    Serial.println(ar.resultCode());
  }

  // if lower/upper targets changed, save to eeprom.
  saveLTHIfChanged();
  saveUTHIfChanged();

  uint32_t now = millis();
  if (now - lastCheck > serverdata.checkInterval) {
    lastCheck = now;
    Serial.println("checking");
    automato.readTempHumidity();
    float humidity = automato.getHumidity();

    // turn on power if below our humidity target
    if (humidity < serverdata.lowertargethumidity) {
      Serial.println("active output: ");
      digitalWrite(output_pin, HIGH);
    }

    // turn off power if above our target.
    if (humidity > serverdata.uppertargethumidity) {
      Serial.println("deactive output: ");
      digitalWrite(output_pin, LOW);
    }
  }

  serverdata.loops++;
}
