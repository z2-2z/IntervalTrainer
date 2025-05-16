#!/bin/bash
set -e

export ANDROID_HOME=/home/z2_/.android/AndroidSdk
export ANDROID_NDK_HOME=/home/z2_/.android/AndroidSdk/ndk/29.0.13113456

if [[ -z "$(archlinux-java status | grep default | grep java-24-openjdk)" ]];
then
    echo "java-24-openjdk not selected as default version" 
    exit 1
fi

dx bundle -r --arch arm64 --platform android

cd target/dx/interval-training/release/android/app/app/build/outputs/bundle/release
java -jar ~/Downloads/bundletool-all-1.18.1.jar build-apks --bundle=./IntervalTraining-aarch64.aab --output=./IntervalTraining.apks --mode=universal
unzip IntervalTraining.apks
test -f universal.apk
mv universal.apk ../../../../../../../../../../../IntervalTraining.apk
