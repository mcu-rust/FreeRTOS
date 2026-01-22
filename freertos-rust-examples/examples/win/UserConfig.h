/*
 * FreeRTOS Kernel V10.3.0
 * Copyright (C) 2020 Amazon.com, Inc. or its affiliates.  All Rights Reserved.
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
 */

#ifndef FREERTOS_CONFIG_H
#define FREERTOS_CONFIG_H

/*-----------------------------------------------------------
 * Application specific definitions.
 *
 * These definitions should be adjusted for your particular hardware and
 * application requirements.
 *
 * THESE PARAMETERS ARE DESCRIBED WITHIN THE 'CONFIGURATION' SECTION OF THE
 * FreeRTOS API DOCUMENTATION AVAILABLE ON THE FreeRTOS.org WEB SITE.  See
 * http://www.freertos.org/a00110.html
 *----------------------------------------------------------*/

#define configUSE_DAEMON_TASK_STARTUP_HOOK 1

/* In this simulated case, the stack only has to hold one small structure as the real stack
    is part of the win32 thread. */
#define configUSE_ALTERNATIVE_API                     0
#define configUSE_QUEUE_SETS                          1
#define configSUPPORT_STATIC_ALLOCATION               1
#define configINITIAL_TICK_COUNT                      ((TickType_t)0) /* For test. */
#define configSTREAM_BUFFER_TRIGGER_LEVEL_TEST_MARGIN 1               /* As there are a lot of tasks running. */

/* Run time stats gathering configuration options. */
unsigned long ulGetRunTimeCounterValue(void); /* Prototype of function that returns run time counter. */
void vConfigureTimerForRunTimeStats(void);    /* Prototype of function that initialises the run time counter. */
#define configGENERATE_RUN_TIME_STATS            0
#define portCONFIGURE_TIMER_FOR_RUN_TIME_STATS() vConfigureTimerForRunTimeStats()
#define portGET_RUN_TIME_COUNTER_VALUE()         ulGetRunTimeCounterValue()

#define configINCLUDE_MESSAGE_BUFFER_AMP_DEMO 0
#if (configINCLUDE_MESSAGE_BUFFER_AMP_DEMO == 1)
extern void vGenerateCoreBInterrupt(void *xUpdatedMessageBuffer);
#    define sbSEND_COMPLETED(pxStreamBuffer) vGenerateCoreBInterrupt(pxStreamBuffer)
#endif /* configINCLUDE_MESSAGE_BUFFER_AMP_DEMO */

/* Include the FreeRTOS+Trace FreeRTOS trace macro definitions. */
#if configUSE_TRACE_FACILITY == 1
// #include "trcRecorder.h"
#endif

#endif /* FREERTOS_CONFIG_H */
