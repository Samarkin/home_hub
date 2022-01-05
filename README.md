# Home Hub

Backend service for Home Portal.

## What?

This service collects and groups data required by Home Portal, and processes
client requests.

## How to use?

### Prerequisites

1. [govee_collector](https://github.com/Samarkin/govee_collector) must be running
on the same machine.


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
