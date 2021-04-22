// Generated by gir (https://github.com/gtk-rs/gir @ a972bd6)
// from gir-files (https://github.com/gtk-rs/gir-files @ 6088bb6)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 208138a)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GST_INTERPOLATION_MODE_CUBIC);
    PRINT_CONSTANT((gint) GST_INTERPOLATION_MODE_CUBIC_MONOTONIC);
    PRINT_CONSTANT((gint) GST_INTERPOLATION_MODE_LINEAR);
    PRINT_CONSTANT((gint) GST_INTERPOLATION_MODE_NONE);
    PRINT_CONSTANT((gint) GST_LFO_WAVEFORM_REVERSE_SAW);
    PRINT_CONSTANT((gint) GST_LFO_WAVEFORM_SAW);
    PRINT_CONSTANT((gint) GST_LFO_WAVEFORM_SINE);
    PRINT_CONSTANT((gint) GST_LFO_WAVEFORM_SQUARE);
    PRINT_CONSTANT((gint) GST_LFO_WAVEFORM_TRIANGLE);
    return 0;
}
