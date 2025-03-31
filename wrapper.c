#include "wrapper.h"

SERIALINFO find_model_info(const char *model) {
    SERIALINFO info = {
      .modelIndex  = -1,
      .decodedYear = -1,
      .decodedWeek = -1,
      .decodedCopy = -1,
      .decodedLine = -1
    };
    for (int32_t j = 0; j < APPLE_MODEL_MAX; j++) {
        if (!strcmp(model, ApplePlatformData[j].productName)) {
            info.modelIndex = j;
            break;
        }
    }
    return info;
}

SerialResult find_serial_mlb(SERIALINFO *info) {
    SerialResult result = {0};
    if (get_serial(info)) {
        get_mlb(info, result.mlb, MLB_MAX_SIZE);
        snprintf(result.serial, SERIAL_MAX_SIZE, "%s%s%s%s%s",info->country, info->year, info->week, info->line, info->model);
    }
    return result;
}
