#ifndef __FREERTOS_CONFIG_INNER_H__
#define __FREERTOS_CONFIG_INNER_H__

#include <stdbool.h>
#include <stdint.h>

#include "UserConfig.h"

#ifdef __IS_CORTEX_M
#    define vPortSVCHandler     SVCall
#    define xPortPendSVHandler  PendSV
#    define xPortSysTickHandler SysTick
#    define SVC_Handler         SVCall
#    define SysTick_Handler     SysTick
#    define PendSV_Handler      PendSV
#endif

extern void assert_callback(uint32_t ulLine, const char *const pcFileName);
#define configASSERT(x) \
    if ((x) == 0)       \
    assert_callback(__LINE__, __FILE__)

#ifndef configTICK_RATE_HZ
#    define configTICK_RATE_HZ ((TickType_t)1000)  // 1000=1ms per tick, 100=10ms per tick
#endif

#define configUSE_IDLE_HOOK                  0
#define configUSE_TICK_HOOK                  0
#define configUSE_16_BIT_TICKS               0
#define configUSE_MALLOC_FAILED_HOOK         0
#define configUSE_STATS_FORMATTING_FUNCTIONS 0
#define configUSE_MUTEXES                    1
#define configUSE_APPLICATION_TASK_TAG       0

/* Co-routine definitions. */
#define configUSE_CO_ROUTINES           0
#define configMAX_CO_ROUTINE_PRIORITIES (2)

#define INCLUDE_vTaskDelay                1
#define INCLUDE_vTaskPrioritySet          0
#define INCLUDE_uxTaskPriorityGet         0
#define INCLUDE_eTaskGetState             0
#define INCLUDE_xTaskGetCurrentTaskHandle 1
#define configUSE_TASK_NOTIFICATIONS      1

#endif
