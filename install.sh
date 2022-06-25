#!/bin/bash

cargo build -r
sudo cp target/release/weather_cli /bin

