#include <stdint.h>

struct date {
    uint16_t day_of_month;
    uint16_t month;
    uint16_t year;
};

uint32_t convert(struct date date) {
    uint32_t year = date.year;
    uint32_t month = date.month;
    uint32_t day = date.day_of_month;

    uint32_t a = (14 - month) / 12;
    uint32_t y = year + 4800 - a;
    uint32_t m = month + 12 * a - 3;

    return day + (153 * m + 2) / 5 + y * 365 + y / 4 - y / 100 + y / 400 - 32045;
}
