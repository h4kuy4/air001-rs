extern "C" {
    pub fn NMI_Handler();               
    pub fn HardFault_Handler();        
    pub fn SVCall_Handler();           
    pub fn PendSV_Handler();            
    pub fn SysTick_Handler();           
    pub fn WWDG_IRQ();                  
    pub fn PVD_IRQ();                   
    pub fn RTC_IRQ();                   
    pub fn Flash_IRQ();                 
    pub fn RCC_IRQ();                   
    pub fn EXTI0_1_IRQ();               
    pub fn EXTI2_3_IRQ();               
    pub fn EXIT4_15_IRQ();              
    pub fn DMA_Channel1_IRQ();          
    pub fn DMA_Channel2_3_IRQ();        
    pub fn ADC_COMP_IRQ();              
    pub fn TIM1_BRK_UP_TRG_COM_IRQ();   
    pub fn TIM1_CC_IRQ();               
    pub fn TIM3_IRQ();                  
    pub fn LPTIM_IRQ();                 
    pub fn TIM14_IRQ();                 
    pub fn TIM16_IRQ();                 
    pub fn TIM17_IRQ();                 
    pub fn I2C1_IRQ();                  
    pub fn SPI1_IRQ();                  
    pub fn SPI2_IRQ();                  
    pub fn USQRT1_IRQ();                
    pub fn USQRT2_IRQ();                
    pub fn LED_IRQ();                   
}
