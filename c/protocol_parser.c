#include <stdio.h>

typedef struct {
    unsigned int device_id : 4;
    unsigned int status : 1;
    unsigned int temp : 27;
} SensorData;

void decodeSensor(unsigned int raw_data) {

    SensorData *data = (SensorData *)&raw_data;

    printf("--- Decoding Raw Data: 0x%X ---\n", raw_data);
    printf("Device ID: %u\n", data->device_id);
    printf("Status   : %s\n", data->status ? "ON" : "OFF");
    printf("Temp Raw : %u\n", data->temp);
}

int main() {
    unsigned int incoming_packet = 0x1ABCDE2F;
    decodeSensor(incoming_packet);
    return 0;
}
