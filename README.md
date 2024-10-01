# PlayBridge

Simple ADB emulator for Google Play Games, especially for Arknights & MAA

## Download

- You can get latest builds at [release page](https://github.com/ACK72/PlayBridge/releases/latest)

## How to use
![setting](https://github.com/ACK72/PlayBridge/assets/25812442/69f980b6-7c9e-4a93-b1b5-f2a21c1b0680)

- Change the settings in MAA as shown above.
- Make an .cmd script with below code where MAA.exe exists. (If you don't want to run script each time, you can use setx to permanently set env variables.)

```
set PLAYBRIDGE_TITLE=명일방주
set PLAYBRIDGE_QUICK=1
set PLAYBRIDGE_DEBUG=1
MAA.exe
```

- Change PLAYBRIDGE_TITLE with the title you are running. For example, set it to Arknights for YOSTAR_EN. (You can also use it for other games.)
- If you don't want quick swipe or debug mode, delete each corresponding line in the .cmd script.

## Limitations

- Of the special keys, currently, only ESC key input supported. This may be updated in the future.
- (Arknights & MAA only) All features are currently working, but due to differences in in-game graphics between Android and Google Play Games Beta, MAA may not work smoothly in certain situations.