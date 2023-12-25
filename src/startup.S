.section .isr_vec_table

.align 2

.long Reset_Handler             
.long NMI_Handler               
.long HardFault_Handler         

.long 0                         
.long 0                         
.long 0                         
.long 0                         

.long 0                         
.long 0                         
.long 0                         
.long SVCall_Handler            

.long 0                         
.long 0                         
.long PendSV_Handler            
.long SysTick_Handler           

.long WWDG_IRQ                  
.long PVD_IRQ                   
.long RTC_IRQ                   
.long Flash_IRQ                 

.long RCC_IRQ                   
.long EXTI0_1_IRQ               
.long EXTI2_3_IRQ               
.long EXIT4_15_IRQ              

.long 0                         
.long DMA_Channel1_IRQ          
.long DMA_Channel2_3_IRQ        
.long 0                         

.long ADC_COMP_IRQ              
.long TIM1_BRK_UP_TRG_COM_IRQ   
.long TIM1_CC_IRQ               
.long 0                         

.long TIM3_IRQ                  
.long LPTIM_IRQ                 
.long 0                         
.long TIM14_IRQ                 

.long 0                         
.long TIM16_IRQ                 
.long TIM17_IRQ                 
.long I2C1_IRQ                  

.long 0                         
.long SPI1_IRQ                  
.long SPI2_IRQ                  
.long USQRT1_IRQ                

.long USQRT2_IRQ                
.long 0                         
.long LED_IRQ                   
.long 0                         

.text
.align

.global Reset_Handler

inf_loop:
    b inf_loop

.thumb_func
Reset_Handler:
    b main
    b inf_loop

.thumb_func
HardFault_Handler:
    b fault
    b inf_loop
