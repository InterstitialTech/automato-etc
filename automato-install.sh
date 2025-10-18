
arduino-cli core install esp32:esp32@1.0.6 --additional-urls https://raw.githubusercontent.com/espressif/arduino-esp32/gh-pages/package_esp32_index.json

arduino-cli core install automato:esp32 --additional-urls https://raw.githubusercontent.com/InterstitialTech/automato-arduino/refs/heads/main/package_automato_index.json

arduino-cli lib install "Adafruit ILI9341"

arduino-cli lib install "Adafruit GFX Library"

arduino-cli lib install "SparkFun SHTC3 Humidity and Temperature Sensor Library"


wget http://www.airspayce.com/mikem/arduino/RadioHead/RadioHead-1.120.zip
mv RadioHead-1.120.zip ~/Arduino/libraries/
unzip ~/Arduino/libraries/RadioHead-1.120.zip -d ~/Arduino/libraries/

arduino-cli lib install "SparkFun SHTC3 Humidity and Temperature Sensor Library"

#  nixos:
#
#  nix-shell -p python38Packages.pyserial


arduino-cli compile -b automato:esp32:automato automato-arduino/example-sketches/automato_helloWorld/
