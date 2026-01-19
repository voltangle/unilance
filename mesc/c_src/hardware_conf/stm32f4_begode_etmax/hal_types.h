// This file is used to generate HAL types for Rust, which are then passed to MESC.
// Contents of this file were directly copied from several files in the STM32CubeF4
// package. The exact sources are specified lower.

#include <stdint.h>

#include "core_cm4.h"
#include "stm32f405xx.h"

#pragma once

/* ########################################
 * ST HAL stm32f4xx.h
 * ########################################
 */

#define SET_BIT(REG, BIT) ((REG) |= (BIT))

#define CLEAR_BIT(REG, BIT) ((REG) &= ~(BIT))

#define READ_BIT(REG, BIT) ((REG) & (BIT))

#define CLEAR_REG(REG) ((REG) = (0x0))

#define WRITE_REG(REG, VAL) ((REG) = (VAL))

#define READ_REG(REG) ((REG))

#define MODIFY_REG(REG, CLEARMASK, SETMASK) \
    WRITE_REG((REG), (((READ_REG(REG)) & (~(CLEARMASK))) | (SETMASK)))

typedef enum { RESET = 0U, SET = !RESET } FlagStatus, ITStatus;

typedef enum { DISABLE = 0U, ENABLE = !DISABLE } FunctionalState;
#define IS_FUNCTIONAL_STATE(STATE) (((STATE) == DISABLE) || ((STATE) == ENABLE))

typedef enum { SUCCESS = 0U, ERROR = !SUCCESS } ErrorStatus;

/* ########################################
 * ST HAL stm32f4xx_hal_def.h
 * ########################################
 */

/**
 * @brief  HAL Status structures definition
 */
typedef enum {
    HAL_OK = 0x00U,
    HAL_ERROR = 0x01U,
    HAL_BUSY = 0x02U,
    HAL_TIMEOUT = 0x03U
} HAL_StatusTypeDef;

/**
 * @brief  HAL Lock structures definition
 */
typedef enum { HAL_UNLOCKED = 0x00U, HAL_LOCKED = 0x01U } HAL_LockTypeDef;

/* ########################################
 * ST HAL stm32f4xx_hal_dma.h
 * ########################################
 */

/**
 * @brief  HAL DMA State structures definition
 */
typedef enum {
    HAL_DMA_STATE_RESET = 0x00U,   /*!< DMA not yet initialized or disabled */
    HAL_DMA_STATE_READY = 0x01U,   /*!< DMA initialized and ready for use   */
    HAL_DMA_STATE_BUSY = 0x02U,    /*!< DMA process is ongoing              */
    HAL_DMA_STATE_TIMEOUT = 0x03U, /*!< DMA timeout state                   */
    HAL_DMA_STATE_ERROR = 0x04U,   /*!< DMA error state                     */
    HAL_DMA_STATE_ABORT = 0x05U,   /*!< DMA Abort state                     */
} HAL_DMA_StateTypeDef;

/**
 * @brief  DMA Configuration Structure definition
 */
typedef struct {
    uint32_t Channel; /*!< Specifies the channel used for the specified stream.
                           This parameter can be a value of @ref DMA_Channel_selection */

    uint32_t Direction; /*!< Specifies if the data will be transferred from memory to
                           peripheral, from memory to memory or from peripheral to memory.
                             This parameter can be a value of @ref
                           DMA_Data_transfer_direction              */

    uint32_t PeriphInc; /*!< Specifies whether the Peripheral address register should be
                           incremented or not. This parameter can be a value of @ref
                           DMA_Peripheral_incremented_mode          */

    uint32_t MemInc; /*!< Specifies whether the memory address register should be
                        incremented or not. This parameter can be a value of @ref
                        DMA_Memory_incremented_mode              */

    uint32_t PeriphDataAlignment; /*!< Specifies the Peripheral data width.
                                       This parameter can be a value of @ref
                                     DMA_Peripheral_data_size                 */

    uint32_t MemDataAlignment; /*!< Specifies the Memory data width.
                                    This parameter can be a value of @ref
                                  DMA_Memory_data_size                     */

    uint32_t
        Mode; /*!< Specifies the operation mode of the DMAy Streamx.
                   This parameter can be a value of @ref DMA_mode
                   @note The circular buffer mode cannot be used if the memory-to-memory
                         data transfer is configured on the selected Stream */

    uint32_t Priority; /*!< Specifies the software priority for the DMAy Streamx.
                            This parameter can be a value of @ref DMA_Priority_level */

    uint32_t FIFOMode; /*!< Specifies if the FIFO mode or Direct mode will be used for the
                          specified stream. This parameter can be a value of @ref
                          DMA_FIFO_direct_mode
                            @note The Direct mode (FIFO mode disabled) cannot be used if
                          the memory-to-memory data transfer is configured on the selected
                          stream       */

    uint32_t FIFOThreshold; /*!< Specifies the FIFO threshold level.
                                 This parameter can be a value of @ref
                               DMA_FIFO_threshold_level                  */

    uint32_t MemBurst; /*!< Specifies the Burst transfer configuration for the memory
                          transfers. It specifies the amount of data to be transferred in
                          a single non interruptible transaction. This parameter can be a
                          value of @ref DMA_Memory_burst
                            @note The burst mode is possible only if the address Increment
                          mode is enabled. */

    uint32_t PeriphBurst; /*!< Specifies the Burst transfer configuration for the
                             peripheral transfers. It specifies the amount of data to be
                             transferred in a single non interruptible transaction. This
                             parameter can be a value of @ref DMA_Peripheral_burst
                               @note The burst mode is possible only if the address
                             Increment mode is enabled. */
} DMA_InitTypeDef;

/**
 * @brief  DMA handle Structure definition
 */
typedef struct __DMA_HandleTypeDef {
    DMA_Stream_TypeDef* Instance;    /*!< Register base address                  */
    DMA_InitTypeDef Init;            /*!< DMA communication parameters           */
    HAL_LockTypeDef Lock;            /*!< DMA locking object                     */
    __IO HAL_DMA_StateTypeDef State; /*!< DMA transfer state                     */
    void* Parent;                    /*!< Parent object state                    */
    void (*XferCpltCallback)(
        struct __DMA_HandleTypeDef* hdma); /*!< DMA transfer complete callback         */
    void (*XferHalfCpltCallback)(
        struct __DMA_HandleTypeDef* hdma); /*!< DMA Half transfer complete callback    */
    void (*XferM1CpltCallback)(
        struct __DMA_HandleTypeDef* hdma); /*!< DMA transfer complete Memory1 callback */
    void (*XferM1HalfCpltCallback)(
        struct __DMA_HandleTypeDef*
            hdma); /*!< DMA transfer Half complete Memory1 callback */
    void (*XferErrorCallback)(
        struct __DMA_HandleTypeDef* hdma); /*!< DMA transfer error callback            */
    void (*XferAbortCallback)(
        struct __DMA_HandleTypeDef* hdma); /*!< DMA transfer Abort callback            */
    __IO uint32_t ErrorCode;               /*!< DMA Error code                          */
    uint32_t StreamBaseAddress;            /*!< DMA Stream Base Address                */
    uint32_t StreamIndex;                  /*!< DMA Stream Index                       */
} DMA_HandleTypeDef;

/* ########################################
 * ST HAL stm32f4xx_hal_tim.h
 * ########################################
 */

/** @defgroup TIM_Output_Compare_and_PWM_modes TIM Output Compare and PWM Modes
 * @{
 */
#define TIM_OCMODE_TIMING 0x00000000U      /*!< Frozen                                 */
#define TIM_OCMODE_ACTIVE TIM_CCMR1_OC1M_0 /*!< Set channel to active level on match */
#define TIM_OCMODE_INACTIVE \
    TIM_CCMR1_OC1M_1 /*!< Set channel to inactive level on match */
#define TIM_OCMODE_TOGGLE \
    (TIM_CCMR1_OC1M_1 | TIM_CCMR1_OC1M_0) /*!< Toggle                                 */
#define TIM_OCMODE_PWM1 \
    (TIM_CCMR1_OC1M_2 | TIM_CCMR1_OC1M_1) /*!< PWM mode 1                             */
#define TIM_OCMODE_PWM2 \
    (TIM_CCMR1_OC1M_2 | TIM_CCMR1_OC1M_1 | TIM_CCMR1_OC1M_0) /*!< PWM mode 2 */
#define TIM_OCMODE_FORCED_ACTIVE \
    (TIM_CCMR1_OC1M_2 | TIM_CCMR1_OC1M_0) /*!< Force active level                     */
#define TIM_OCMODE_FORCED_INACTIVE \
    TIM_CCMR1_OC1M_2 /*!< Force inactive level                   */

/** @defgroup TIM_Interrupt_definition TIM interrupt Definition
 * @{
 */
#define TIM_IT_UPDATE TIM_DIER_UIE  /*!< Update interrupt            */
#define TIM_IT_CC1 TIM_DIER_CC1IE   /*!< Capture/Compare 1 interrupt */
#define TIM_IT_CC2 TIM_DIER_CC2IE   /*!< Capture/Compare 2 interrupt */
#define TIM_IT_CC3 TIM_DIER_CC3IE   /*!< Capture/Compare 3 interrupt */
#define TIM_IT_CC4 TIM_DIER_CC4IE   /*!< Capture/Compare 4 interrupt */
#define TIM_IT_COM TIM_DIER_COMIE   /*!< Commutation interrupt       */
#define TIM_IT_TRIGGER TIM_DIER_TIE /*!< Trigger interrupt           */
#define TIM_IT_BREAK TIM_DIER_BIE   /*!< Break interrupt             */

/** @brief  Enable the specified TIM interrupt.
 * @param  __HANDLE__ specifies the TIM Handle.
 * @param  __INTERRUPT__ specifies the TIM interrupt source to enable.
 *          This parameter can be one of the following values:
 *            @arg TIM_IT_UPDATE: Update interrupt
 *            @arg TIM_IT_CC1:   Capture/Compare 1 interrupt
 *            @arg TIM_IT_CC2:  Capture/Compare 2 interrupt
 *            @arg TIM_IT_CC3:  Capture/Compare 3 interrupt
 *            @arg TIM_IT_CC4:  Capture/Compare 4 interrupt
 *            @arg TIM_IT_COM:   Commutation interrupt
 *            @arg TIM_IT_TRIGGER: Trigger interrupt
 *            @arg TIM_IT_BREAK: Break interrupt
 * @retval None
 */
#define __HAL_TIM_ENABLE_IT(__HANDLE__, __INTERRUPT__) \
    ((__HANDLE__)->Instance->DIER |= (__INTERRUPT__))

/**
 * @brief  Set the TIM Autoreload Register value on runtime without calling another time
 * any Init function.
 * @param  __HANDLE__ TIM handle.
 * @param  __AUTORELOAD__ specifies the Counter register new value.
 * @retval None
 */
#define __HAL_TIM_SET_AUTORELOAD(__HANDLE__, __AUTORELOAD__) \
    do {                                                     \
        (__HANDLE__)->Instance->ARR = (__AUTORELOAD__);      \
        (__HANDLE__)->Init.Period = (__AUTORELOAD__);        \
    } while (0)

/**
 * @brief  Set the TIM Prescaler on runtime.
 * @param  __HANDLE__ TIM handle.
 * @param  __PRESC__ specifies the Prescaler new value.
 * @retval None
 */
#define __HAL_TIM_SET_PRESCALER(__HANDLE__, __PRESC__) \
    ((__HANDLE__)->Instance->PSC = (__PRESC__))

/** @brief  Disable the specified TIM interrupt.
 * @param  __HANDLE__ specifies the TIM Handle.
 * @param  __INTERRUPT__ specifies the TIM interrupt source to disable.
 *          This parameter can be one of the following values:
 *            @arg TIM_IT_UPDATE: Update interrupt
 *            @arg TIM_IT_CC1:   Capture/Compare 1 interrupt
 *            @arg TIM_IT_CC2:  Capture/Compare 2 interrupt
 *            @arg TIM_IT_CC3:  Capture/Compare 3 interrupt
 *            @arg TIM_IT_CC4:  Capture/Compare 4 interrupt
 *            @arg TIM_IT_COM:   Commutation interrupt
 *            @arg TIM_IT_TRIGGER: Trigger interrupt
 *            @arg TIM_IT_BREAK: Break interrupt
 * @retval None
 */
#define __HAL_TIM_DISABLE_IT(__HANDLE__, __INTERRUPT__) \
    ((__HANDLE__)->Instance->DIER &= ~(__INTERRUPT__))

/**
 * @brief  TIM Time base Configuration Structure definition
 */
typedef struct {
    uint32_t Prescaler; /*!< Specifies the prescaler value used to divide the TIM clock.
                             This parameter can be a number between Min_Data = 0x0000 and
                           Max_Data = 0xFFFF */

    uint32_t CounterMode; /*!< Specifies the counter mode.
                               This parameter can be a value of @ref TIM_Counter_Mode */

    uint32_t Period; /*!< Specifies the period value to be loaded into the active
                          Auto-Reload Register at the next update event.
                          This parameter can be a number between Min_Data = 0x0000 and
                        Max_Data = 0xFFFF.  */

    uint32_t
        ClockDivision; /*!< Specifies the clock division.
                            This parameter can be a value of @ref TIM_ClockDivision */

    uint32_t RepetitionCounter; /*!< Specifies the repetition counter value. Each time the
                                   RCR downcounter reaches zero, an update event is
                                   generated and counting restarts from the RCR value (N).
                                     This means in PWM mode that (N+1) corresponds to:
                                         - the number of PWM periods in edge-aligned mode
                                         - the number of half PWM period in center-aligned
                                   mode GP timers: this parameter must be a number between
                                   Min_Data = 0x00 and Max_Data = 0xFF. Advanced timers:
                                   this parameter must be a number between Min_Data =
                                   0x0000 and Max_Data = 0xFFFF. */

    uint32_t AutoReloadPreload; /*!< Specifies the auto-reload preload.
                                    This parameter can be a value of @ref
                                   TIM_AutoReloadPreload */
} TIM_Base_InitTypeDef;
/**
 * @brief  HAL State structures definition
 */
typedef enum {
    HAL_TIM_STATE_RESET = 0x00U,   /*!< Peripheral not yet initialized or disabled  */
    HAL_TIM_STATE_READY = 0x01U,   /*!< Peripheral Initialized and ready for use    */
    HAL_TIM_STATE_BUSY = 0x02U,    /*!< An internal process is ongoing              */
    HAL_TIM_STATE_TIMEOUT = 0x03U, /*!< Timeout state                               */
    HAL_TIM_STATE_ERROR = 0x04U    /*!< Reception process is ongoing                */
} HAL_TIM_StateTypeDef;

/**
 * @brief  TIM Channel States definition
 */
typedef enum {
    HAL_TIM_CHANNEL_STATE_RESET = 0x00U, /*!< TIM Channel initial state */
    HAL_TIM_CHANNEL_STATE_READY = 0x01U, /*!< TIM Channel ready for use */
    HAL_TIM_CHANNEL_STATE_BUSY =
        0x02U, /*!< An internal process is ongoing on the TIM channel */
} HAL_TIM_ChannelStateTypeDef;

/**
 * @brief  DMA Burst States definition
 */
typedef enum {
    HAL_DMA_BURST_STATE_RESET = 0x00U, /*!< DMA Burst initial state */
    HAL_DMA_BURST_STATE_READY = 0x01U, /*!< DMA Burst ready for use */
    HAL_DMA_BURST_STATE_BUSY = 0x02U,  /*!< Ongoing DMA Burst       */
} HAL_TIM_DMABurstStateTypeDef;

/**
 * @brief  HAL Active channel structures definition
 */
typedef enum {
    HAL_TIM_ACTIVE_CHANNEL_1 = 0x01U,      /*!< The active channel is 1     */
    HAL_TIM_ACTIVE_CHANNEL_2 = 0x02U,      /*!< The active channel is 2     */
    HAL_TIM_ACTIVE_CHANNEL_3 = 0x04U,      /*!< The active channel is 3     */
    HAL_TIM_ACTIVE_CHANNEL_4 = 0x08U,      /*!< The active channel is 4     */
    HAL_TIM_ACTIVE_CHANNEL_CLEARED = 0x00U /*!< All active channels cleared */
} HAL_TIM_ActiveChannel;

typedef struct {
    TIM_TypeDef* Instance;     /*!< Register base address                             */
    TIM_Base_InitTypeDef Init; /*!< TIM Time Base required parameters                 */
    HAL_TIM_ActiveChannel Channel; /*!< Active channel */
    DMA_HandleTypeDef* hdma[7];    /*!< DMA Handlers array
                                        This array is accessed by a @ref DMA_Handle_index */
    HAL_LockTypeDef Lock; /*!< Locking object                                    */
    __IO HAL_TIM_StateTypeDef State;                  /*!< TIM operation state                  */
    __IO HAL_TIM_ChannelStateTypeDef ChannelState[4]; /*!< TIM channel operation state */
    __IO HAL_TIM_ChannelStateTypeDef
        ChannelNState[4]; /*!< TIM complementary channel operation state         */
    __IO HAL_TIM_DMABurstStateTypeDef DMABurstState; /*!< DMA burst operation state */
} TIM_HandleTypeDef;

/*
 * ST HAL stm32f4xx_hal_adc.h
 *
 */

/**
 * @brief  Structure definition of ADC and regular group initialization
 * @note   Parameters of this structure are shared within 2 scopes:
 *          - Scope entire ADC (affects regular and injected groups): ClockPrescaler,
 * Resolution, ScanConvMode, DataAlign, ScanConvMode, EOCSelection, LowPowerAutoWait,
 * LowPowerAutoPowerOff, ChannelsBank.
 *          - Scope regular group: ContinuousConvMode, NbrOfConversion,
 * DiscontinuousConvMode, NbrOfDiscConversion, ExternalTrigConvEdge, ExternalTrigConv.
 * @note   The setting of these parameters with function HAL_ADC_Init() is conditioned to
 * ADC state. ADC state can be either:
 *          - For all parameters: ADC disabled
 *          - For all parameters except 'Resolution', 'ScanConvMode',
 * 'DiscontinuousConvMode', 'NbrOfDiscConversion' : ADC enabled without conversion on
 * going on regular group.
 *          - For parameters 'ExternalTrigConv' and 'ExternalTrigConvEdge': ADC enabled,
 * even with conversion on going. If ADC is not in the appropriate state to modify some
 * parameters, these parameters setting is bypassed without error reporting (as it can be
 * the expected behaviour in case of intended action to update another parameter (which
 * fulfills the ADC state condition) on the fly).
 */
typedef struct {
    uint32_t
        ClockPrescaler;  /*!< Select ADC clock prescaler. The clock is common for
                              all the ADCs.
                              This parameter can be a value of @ref ADC_ClockPrescaler */
    uint32_t Resolution; /*!< Configures the ADC resolution.
                              This parameter can be a value of @ref ADC_Resolution */
    uint32_t DataAlign; /*!< Specifies ADC data alignment to right (MSB on register bit 11
                           and LSB on register bit 0) (default setting) or to left (if
                           regular group: MSB on register bit 15 and LSB on register bit
                           4, if injected group (MSB kept as signed value due to potential
                           negative value after offset application): MSB on register bit
                           14 and LSB on register bit 3). This parameter can be a value of
                           @ref ADC_Data_align */
    uint32_t ScanConvMode; /*!< Configures the sequencer of regular and injected groups.
                                This parameter can be associated to parameter
                              'DiscontinuousConvMode' to have main sequence subdivided in
                              successive parts. If disabled: Conversion is performed in
                              single mode (one channel converted, the one defined in rank
                              1). Parameters 'NbrOfConversion' and
                              'InjectedNbrOfConversion' are discarded (equivalent to set
                              to 1). If enabled:  Conversions are performed in sequence
                              mode (multiple ranks defined by
                              'NbrOfConversion'/'InjectedNbrOfConversion' and each channel
                              rank). Scan direction is upward: from rank1 to rank 'n'.
                                This parameter can be set to ENABLE or DISABLE */
    uint32_t
        EOCSelection; /*!< Specifies what EOC (End Of Conversion) flag is used for
                         conversion by polling and interruption: end of conversion of each
                         rank or complete sequence. This parameter can be a value of @ref
                         ADC_EOCSelection. Note: For injected group, end of conversion
                         (flag&IT) is raised only at the end of the sequence. Therefore,
                         if end of conversion is set to end of each conversion, injected
                         group should not be used with interruption
                         (HAL_ADCEx_InjectedStart_IT) or polling (HAL_ADCEx_InjectedStart
                         and HAL_ADCEx_InjectedPollForConversion). By the way, polling is
                         still possible since driver will use an estimated timing for end
                         of injected conversion. Note: If overrun feature is intended to
                         be used, use ADC in mode 'interruption' (function
                         HAL_ADC_Start_IT() ) with parameter EOCSelection set to end of
                         each conversion or in mode 'transfer by DMA' (function
                         HAL_ADC_Start_DMA()). If overrun feature is intended to be
                         bypassed, use ADC in mode 'polling' or 'interruption' with
                         parameter EOCSelection must be set to end of sequence */
    FunctionalState
        ContinuousConvMode;   /*!< Specifies whether the conversion is performed in single
                                 mode (one conversion) or continuous mode for regular group,
                                   after the selected trigger occurred (software start or
                                 external trigger).   This parameter can be set to ENABLE or
                                 DISABLE. */
    uint32_t NbrOfConversion; /*!< Specifies the number of ranks that will be converted
                                 within the regular group sequencer. To use regular group
                                 sequencer and convert several ranks, parameter
                                 'ScanConvMode' must be enabled. This parameter must be a
                                 number between Min_Data = 1 and Max_Data = 16. */
    FunctionalState
        DiscontinuousConvMode; /*!< Specifies whether the conversions sequence of regular
                                  group is performed in
                                  Complete-sequence/Discontinuous-sequence (main sequence
                                  subdivided in successive parts). Discontinuous mode is
                                  used only if sequencer is enabled (parameter
                                  'ScanConvMode'). If sequencer is disabled, this
                                  parameter is discarded. Discontinuous mode can be
                                  enabled only if continuous mode is disabled. If
                                  continuous mode is enabled, this parameter setting is
                                  discarded. This parameter can be set to ENABLE or
                                  DISABLE. */
    uint32_t NbrOfDiscConversion;  /*!< Specifies the number of discontinuous conversions
                                      in which the  main sequence of regular group
                                      (parameter NbrOfConversion) will be subdivided.  If
                                      parameter 'DiscontinuousConvMode' is disabled, this
                                      parameter is discarded.  This parameter must be a
                                      number between Min_Data = 1 and Max_Data = 8. */
    uint32_t ExternalTrigConv;     /*!< Selects the external event used to trigger the
                                      conversion start of regular group.     If set to
                                      ADC_SOFTWARE_START, external triggers are disabled.     If
                                      set to external trigger source, triggering is on event
                                      rising edge by default.     This parameter can be a value of
                                      @ref ADC_External_trigger_Source_Regular */
    uint32_t ExternalTrigConvEdge; /*!< Selects the external trigger edge of regular
                                      group. If trigger is set to ADC_SOFTWARE_START, this
                                      parameter is discarded. This parameter can be a
                                      value of @ref ADC_External_trigger_edge_Regular */
    FunctionalState
        DMAContinuousRequests; /*!< Specifies whether the DMA requests are performed in
              one shot mode (DMA transfer stop when number of conversions is reached) or
              in Continuous mode (DMA transfer unlimited, whatever number of conversions).
              Note: In continuous mode, DMA must be configured in circular mode. Otherwise
              an overrun will be triggered when DMA buffer maximum pointer is reached.
              Note: This parameter must be modified when no conversion is on going on both
              regular and injected groups (ADC disabled, or ADC enabled without continuous
              mode or external trigger that could launch a conversion). This parameter can
              be set to ENABLE or DISABLE. */
} ADC_InitTypeDef;

typedef struct {
    ADC_TypeDef* Instance; /*!< Register base address */
    ADC_InitTypeDef Init;  /*!< ADC required parameters */
    __IO uint32_t
        NbrOfCurrentConversionRank; /*!< ADC number of current conversion rank */
    DMA_HandleTypeDef* DMA_Handle;  /*!< Pointer DMA Handler */
    HAL_LockTypeDef Lock;           /*!< ADC locking object */
    __IO uint32_t State;            /*!< ADC communication state */
    __IO uint32_t ErrorCode;        /*!< ADC Error code */
} ADC_HandleTypeDef;

/* ########################################
 * Functions
 * ########################################
 */

void HAL_Delay(volatile uint32_t Delay);
uint32_t HAL_RCC_GetHCLKFreq(void);
HAL_StatusTypeDef HAL_TIM_Base_Start(TIM_HandleTypeDef* htim);
