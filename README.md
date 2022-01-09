# Home Hub

Backend service for Home Portal.

## What?

This service collects and groups data required by Home Portal, and processes
client requests.

## How to use?

### Prerequisites

1. The following services must be running on the same machine:

   * [govee_collector](https://github.com/Samarkin/govee_collector) 
   * [HomeEntertainmentMonitor](https://github.com/Samarkin/HomeEntertainmentMonitor)


### Instructions

1. Checkout the repository

   ```shell
   git clone --recurse-submodules https://github.com/Samarkin/home_hub
   cd home_hub
   ```
2. Build and run:

   ```shell
   cargo run
   ```
