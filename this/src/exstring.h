#pragma once
#include <stdio.h>
#include <stdlib.h>

typedef struct String
{
    size_t len;
    size_t capacity;
    char* str;
}  String;

static void string_append(const char* cstr){

}