#include "OpenCorePkg/Utilities/macserial/macserial.h"
#include "OpenCorePkg/Utilities/macserial/modelinfo_autogen.h"
#include "OpenCorePkg/Utilities/macserial/macserial.c"

#define SERIAL_MAX_SIZE 15

SERIALINFO find_model_info(const char *model);

typedef struct {
    char serial[SERIAL_MAX_SIZE];
    char mlb[MLB_MAX_SIZE];
} SerialResult;

SerialResult find_serial_mlb(SERIALINFO *info);
