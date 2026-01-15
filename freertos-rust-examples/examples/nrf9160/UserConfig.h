/**
 *
 * FreeRTOS Kernel V10.2.0
 * Copyright (C) 2019 Amazon.com, Inc. or its affiliates.  All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * http://www.FreeRTOS.org
 * http://aws.amazon.com/freertos
 *
 * 1 tab == 4 spaces!
 *
 * The FreeRTOS Cortex M33 port can be configured to run on the Secure Side only
 * i.e. the processor boots as secure and never jumps to the non-secure side.
 * The Trust Zone support in the port must be disabled in order to run FreeRTOS
 * on the secure side. The following are the valid configuration seetings:
 *
 * 1. Run FreeRTOS on the Secure Side:
 *		configRUN_FREERTOS_SECURE_ONLY = 1 and configENABLE_TRUSTZONE = 0
 *
 * 2. Run FreeRTOS on the Non-Secure Side with Secure Side function call support:
 *		configRUN_FREERTOS_SECURE_ONLY = 0 and configENABLE_TRUSTZONE = 1
 *
 * 3. Run FreeRTOS on the Non-Secure Side only i.e. no Secure Side function call support:
 *		configRUN_FREERTOS_SECURE_ONLY = 0 and configENABLE_TRUSTZONE = 0
 */

/******************************************************************************
        See http://www.freertos.org/a00110.html for an explanation of the
        definitions contained in this file.
******************************************************************************/

#ifndef FREERTOS_CONFIG_H
#define FREERTOS_CONFIG_H

/*-----------------------------------------------------------
 * Application specific definitions.
 *
 * These definitions should be adjusted for your particular hardware and
 * application requirements.
 *
 * THESE PARAMETERS ARE DESCRIBED WITHIN THE 'CONFIGURATION' SECTION OF THE
 * FreeRTOS API DOCUMENTATION AVAILABLE ON THE FreeRTOS.org WEB SITE.
 * http://www.freertos.org/a00110.html
 *----------------------------------------------------------*/

#define configRUN_FREERTOS_SECURE_ONLY 1
#define configENABLE_TRUSTZONE         0

#define configNUM_THREAD_LOCAL_STORAGE_POINTERS 1

/* Cortex M33 port configuration. */
#define configENABLE_MPU 0
#define configENABLE_FPU 1

#define configSUPPORT_DYNAMIC_ALLOCATION 1

/* Constants related to the behaviour or the scheduler. */
#define configUSE_PREEMPTION    1
#define configUSE_TIME_SLICING  1
#define configMAX_PRIORITIES    (5)
#define configIDLE_SHOULD_YIELD 1

/* Constants that describe the hardware and memory usage. */
#define configCPU_CLOCK_HZ              (64000000UL)  // 64M // from HAL: SystemCoreClock
#define configMINIMAL_STACK_SIZE        ((uint16_t)128)
#define configMINIMAL_SECURE_STACK_SIZE (1024)
#define configMAX_TASK_NAME_LEN         (12)
#define configTOTAL_HEAP_SIZE           ((size_t)(50 * 1024))

/* Constants that build features in or out. */
#define configUSE_TICKLESS_IDLE    0
#define configUSE_NEWLIB_REENTRANT 0
#define configUSE_QUEUE_SETS       0

/* Constants that define which hook (callback) functions should be used. */
#define configUSE_IDLE_HOOK          0
#define configUSE_TICK_HOOK          0
#define configUSE_MALLOC_FAILED_HOOK 0

/* Constants provided for debugging and optimisation assistance. */
#define configCHECK_FOR_STACK_OVERFLOW 2

#define configQUEUE_REGISTRY_SIZE 0

/* Software timer definitions. */
#define configUSE_TIMERS             1
#define configTIMER_TASK_PRIORITY    (3)
#define configTIMER_QUEUE_LENGTH     5
#define configTIMER_TASK_STACK_DEPTH (configMINIMAL_STACK_SIZE)

/* Have FreeRTOS provide an errno variable in task handle.
 * We need errno for bsd_lib */
#define configUSE_POSIX_ERRNO 1

/* Dimensions a buffer that can be used by the FreeRTOS+CLI command interpreter.
 * See the FreeRTOS+CLI documentation for more information:
 * http://www.FreeRTOS.org/FreeRTOS-Plus/FreeRTOS_Plus_CLI/ */
#define configCOMMAND_INT_MAX_OUTPUT_SIZE 2048

/* Interrupt priority configuration follows...................... */

/* Use the system definition, if there is one. */
#ifdef __NVIC_PRIO_BITS
#    define configPRIO_BITS __NVIC_PRIO_BITS
#else
#    define configPRIO_BITS 3 /* 8 priority levels. */
#endif

/* The lowest interrupt priority that can be used in a call to a "set priority"
 * function. */
#define configLIBRARY_LOWEST_INTERRUPT_PRIORITY 0x07

/* The highest interrupt priority that can be used by any interrupt service
 * routine that makes calls to interrupt safe FreeRTOS API functions.  DO NOT
 * CALL INTERRUPT SAFE FREERTOS API FUNCTIONS FROM ANY INTERRUPT THAT HAS A
 * HIGHER PRIORITY THAN THIS! (higher priorities are lower numeric values). */
#define configLIBRARY_MAX_SYSCALL_INTERRUPT_PRIORITY 2

/* Interrupt priorities used by the kernel port layer itself.  These are generic
 * to all Cortex-M ports, and do not rely on any particular library functions. */
#define configKERNEL_INTERRUPT_PRIORITY (configLIBRARY_LOWEST_INTERRUPT_PRIORITY << (8 - configPRIO_BITS))

/* !!!! configMAX_SYSCALL_INTERRUPT_PRIORITY must not be set to zero !!!!
 * See http://www.FreeRTOS.org/RTOS-Cortex-M3-M4.html. */
#define configMAX_SYSCALL_INTERRUPT_PRIORITY (configLIBRARY_MAX_SYSCALL_INTERRUPT_PRIORITY << (8 - configPRIO_BITS))

/* The #ifdef guards against the file being included from IAR assembly files. */
#ifndef __IASMARM__

/* Constants related to the generation of run time stats. */
#    define configGENERATE_RUN_TIME_STATS 0
#    define portCONFIGURE_TIMER_FOR_RUN_TIME_STATS()
#    define portGET_RUN_TIME_COUNTER_VALUE() 0
#    define configTICK_RATE_HZ               ((TickType_t)1000)

#endif /* __IASMARM__ */

/* Enable static allocation. */
// #define configSUPPORT_STATIC_ALLOCATION					1

#endif /* FREERTOS_CONFIG_H */
