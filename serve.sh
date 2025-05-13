#!/bin/bash
set -e

export ANDROID_HOME=/home/z2_/.android/AndroidSdk
export ANDROID_NDK_HOME=/home/z2_/.android/AndroidSdk/ndk/29.0.13113456

if [[ -z "$(archlinux-java status | grep default | grep java-24-openjdk)" ]];
then
    echo "java-24-openjdk not selected as default version" 
    exit 1
fi

dx serve --platform android --hot-reload false
