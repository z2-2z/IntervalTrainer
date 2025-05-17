#!/bin/bash
set -e

export ANDROID_HOME="$HOME/.android/AndroidSdk"
export ANDROID_NDK_HOME="$HOME/.android/AndroidSdk/ndk/29.0.13113456"

if [[ -z "$(archlinux-java status | grep default | grep java-24-openjdk)" ]];
then
    echo "java-24-openjdk not selected as default version" 
    exit 1
fi

#cargo clean -r
dx bundle -r --arch arm64 --platform android

#find target/dx/interval-training/release/android/app/app/src/main/res -name "*.webp" -type f -delete
#rm target/dx/interval-training/release/android/app/app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml
#cp assets/icons/logo.png target/dx/interval-training/release/android/app/app/src/main/res/drawable/splash.png
#cd target/dx/interval-training/release/android/app
#./gradlew clean
#./gradlew assembleRelease
#cd app/build/outputs/apk/release/
#"$ANDROID_HOME/build-tools/35.0.0/zipalign" -v -p 4  app-release-unsigned.apk aligned.apk
#"$ANDROID_HOME/build-tools/35.0.0/apksigner" sign --ks ~/.android/debug.keystore --ks-pass pass:android --out IntervalTraining.apk aligned.apk

cd target/dx/interval-training/release/android/app/app/build/outputs/bundle/release
rm -f ./IntervalTraining.apks
java -jar ~/Downloads/bundletool-all-1.18.1.jar build-apks --bundle=./IntervalTraining-aarch64.aab --output=./IntervalTraining.apks --mode=universal
unzip -o IntervalTraining.apks
test -f universal.apk
mv universal.apk ../../../../../../../../../../../IntervalTraining.apk
