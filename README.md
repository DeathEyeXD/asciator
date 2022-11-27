# asciator - simple program to convert images into ascii art!

Works with most popular image formats such as: png, jpg. webp.
Super easy to use. Results are the best when image is on dark/transparent background. With other images you can convert the colors to make it look a bit better.

## usage:
```
Usage: asciator [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to the image

Options:
      --scale-px <SCALE_PX>
          Scale the image to this width. Cannot be used with scale [default: 100]
      --scale <SCALE>
          Scale the image by this factor. Cannot be used with scale-px
  -c, --colorize
          Print the resulted ASCII art in color. Note: this will only work if the terminal supports ANSI colors
  -b, --brightness-threshold <BRIGHTNESS_THRESHOLD>
          Controls how bright pixels have to be to be converted to ascii. Lower values mean even dark pixels will be converted to ascii [default: 1]
  -h, --help
          Print help information
  -V, --version
          Print version information
```

## Installing

locally (after cloning the repository):

```
cargo install --path .
```

remotely:

```
cargo install --git https://github.com/DeathEyeXD/asciator
```

## example:
```
                                                                                                                                            
                                                                                    !!vv!!LLvv!!                                            
                                                                                ,,vvLLLLLLLLLLvv                                            
                                                                              !!vvLLLLLLLLLLLLvv                                            
                                                                            !!vvLLLLLLLLLLLLLLvv                                            
                                                                          !!vvLLLLLLLLLLLLLLLL!!                                            
                                                                        !!vvLLLLLLLLLLLLLLLL!!vv                                            
                                                                      !!vvLLLLLLLLLLLLLLLLLLvv!!                                            
                                                                    !!vvLLLLLLLLLLvvLLLLLLvv!!                                              
                                                                    vvLLLLLLLLLLLLLLLLLLLLvv                                                
                                                                  !!LLLLLLLLLLLLLLLLLLLLvvvv                                                
                                                                !!vvLLLLLLLLLLLLLLLLLLLLvv                                                  
                                                                vvLLLLLLLLLLLLLLLLLLLLLL..                                                  
                                                              ;;LLLLLLLLLLLLLLLLLLLLLLLL!!                                                  
                                                              vvLLLLLLLLLLLLLLLLLLLLLLLL!!                                                  
                                                            vvLLLLLLLLLLLLLLLLLLLLLLLLLL!!                                                  
                                                              LLLLLLLLLL..LLLLLLLLLLLLLL!!                                                  
                                                              LLLLLLLL  LL  LLLLLLLLLLLL!!                                                  
                                                          ..LLLLLLLLLLLLll    LLLLLLLLLL!!                                                  
                                                        !!vvLLLLLLLLLLLLLLLL  LLLLLLLLLL!!                                                  
                                                        vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL!!                                                  
                                                      vv$$!!LLLLLLLL,,  LLLLLLLLLLLLLLLL!!                                                  
                                                    $$$$$$$$$$LLLL$$$$$$$$  LLLLLLLLLLLL;;                                                  
  !!vvvv!!                                        $$$$$$$$$$$$  $$$$$$$$$$$$LLLLLLLLLLLL!!                                                  
  vvLLLL!!vv!!                                  $$$$$$$$$$$$$$$$$$$$$$$$$$$$  LLLLLLLLLLvv                                                  
  LLLLLLLLLLvv!!                                $$$$$$$$  $$FF$$$$$$$$$$$$$$$$LLLLLLLLLLvv                                        !!vv!!;;  
  LLLLLLLLLLLL!!..                              $$$$$$    ;;  $$$$    $$$$$$$$LLLLLLLLLLvv                                ;;!!vvvvLLLLLLvv  
  LLLLLLLLLLLLLLLL!!                            $$$$$$    $$$$$$$$    $$$$$$$$LLLLLLLLLLvv                              ,,vv!!LLLLLLLLLLll  
  vvLLLLLLLLLLLLLLLLvv;;                        $$$$$$$$$$$$$$$$$$  ..$$$$$$  LLLLLLLLLLvv;;                        !!,,vvLLLLLLLLLLLLLL!!  
  vvLLLLLLLLLLLLLLLLLLvv!!                      $$$$$$$$$$$$$$$$$$$$$$$$$$$$LLLLLLLLLLLLvv!!                      ;;vvllLLLLLLLLLLLLLLLLvv  
  !!LLLLLLLLLLLLLLLLLLLL!!!!                    $$$$$$$$$$$$  $$$$$$$$$$$$$$LLLLLLLLLLLLvvvv                    !!vvLLLLLLLLLLLLLLLLLLLLvv  
  !!vvLLLLLLLLLLLLLLLLLLLLvv!!                !!  $$$$$$$$$$LL$$$$$$$$$$EELLLLLLLLLLLLLL!!!!                ;;vvLLLLLLLLLLLLLLLLLLLLLLvv!!  
    vvLLLLLLLLLLLLLLLLLLLLLL!!!!!!          ,,vvLL  $$$$  LLLL..$$$$$$EELLLLLLLLLLLLLLLLLL;;              !!vvLLLLLLLLLLLLLLLLLLLLLLLLvv    
    ;;vvLLLLLLLLLLLLLLLLLLLLLLLLvv          !!vvLLLLLLLLLLLLLLLLLL,,llLLLLLLLL;;LLLLLLLLLLvv            !!vvLLLLLLLLLLLLLLLLLLLLLLLLvv      
      vvLLLLLLLLLLLLLLLL!!LLLLLLLLvvvv      vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL    LLLLLLLLvv        !!vvLLLLLLLLLLLLLLLLLLLLLLLLLLll;;      
      vv!!LLLLLLLLLLLLLLLLLLLLLLLLLL!!!!    !!LLLLLLLLLLLLLLLLLLLLLLLLLLLL  ,,,,LLLLLLLLLLvv        vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv        
        vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv!!LLLL;;LLLLLLLLLLLLLLLLFF    ,,,,,,,,LLLLLLLLLLLL!!  vvvvLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv          
        ;;vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvvLLLLLLLL,,          ,,,,,,,,,,,,,,  LLLLLLLLLLLLvvvvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLll!!          
          vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLllLLLLLLLLLL,,,,,,,,,,,,,,,,,,,,,,,,,,LLLLLLLLLLLLLLvvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv            
          !!vvLLLLLLLLLLLLLLLLLLLLLLLLLLvvLLLLLLLLvv,,,,,,,,,,,,,,,,,,,,,,,,  LLLLLLLLLLLLvvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv              
            vvLLLLLLLLLLLLLLLLLLLLLLLL!!LLLLLLLLLL  ,,,,,,,,,,,,,,,,,,,,,,,,LLLLLLLLLLLLvvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv;;              
              vvLLLLLLLLLLLLLLLLLLLLLLvvLLLLLLLLLL,,,,,,,,,,,,,,,,,,,,,,,,  LLLLLLLLLLvvLLLLLLLLLLLLLLLLLLLLLLLLvvLLLLLLvv;;                
              ;;LLLLLLLLLLLLLLLLLLLLvvLLLLLLLLLL  ,,,,,,,,,,,,,,,,LLFFFF  LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv                  
              ;;vvLLLLLLLLLLLLLLLLLLvvLLLL  LL  ,,,,,,FFFFFFllFFFFFFFF  LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv;;                  
                  vvLLLLLLLLLLLLLLvvLLLLLLLL  ,,,,,,FFFFFFFFFFFFFFFF  LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv!!                    
                  vvLLLLLLLLLLLLvvLLLLLLLLLLLL..  !!FFFFFFFFFFFF  LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv!!                      
                    vvLLLLLLLLLL!!LLLLLLLLLLLLLLLLLL    ..  LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv                        
                    ;;!!LLLLLLvvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv!!                        
                      vvLLLLvvFFLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv                            
                      ;;vvLL!!LLLLLLLLLLLLLLvvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvvvv                            
                          vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL!!!!                              
                        vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv                                
                      ;;!!LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvvvv                                
                      vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvvLLvv                                
                    ;;FFLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv                                
                    vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvvLLLLLL!!                              
                    vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv                              
                    vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv                              
                    vvLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL!!!!                            
                      LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL!!!!                            
                    LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLvv                              
                    FF!!LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLEE                              
                    vvFF  LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLFF                                
                    FF  FF  LLLLLLLLLLLL..LLvvllLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL  FF  FF                              
                    FFFF  FF;;LLLLLLLLLLLLLLLLLLLL,,LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL  FF  FFFF                              
                    FFFFFF,,FFFF..LLLLLLLLLL,,..LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL  FFllFFFFFFFF                              
                    LLFFFFllll  FFFF..LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL  FFFF  FFFFFFFF..                              
                      FFFFllllllll  FFFF  LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL..FFFF  FFFFFFFFFFFF                                
                      FFFFvvllllllllll  FFFF;;  LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL  FFFFFF  FFFFFFFFFFFFFFFF                                
                      FFFFvvllvvFFllllllllll  FFFFFF,,    ,,LLLLLLLLLL,,    !!FFFFFF  FFFFFFFFFFFFFFFFFFFFFF                                
                        FFFFFFFFvvllllllllllllllllll  ..FFFFFFFFFFFFFFFFFF    FFFFFFFFFFFFFFFFFFFFFFFFFFFF                                  
                        FFFFFFvvllllllllllllllllllllllllllllFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF                                  
                          FFFFvvllllllllllllllvvvvllllllllvvFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFvvvv!!LL                                    
                            FFFFllllllllllllllvvFFllllllllFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFvvllllll..                                    
                              FFFFvvllllllllllvvFFvvvv!!FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFvvllllllll                                      
                          ll..  FFFFFFvv!!!!!!FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFllllllllll                                        
                          EEFFFF  FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFvvllllllll                                          
                          FFFFFFFF    FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFvvllll                                            
                          FFFFFFFFFFFF    FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFvvll                                            
                        LLFFFFFFFFFFFFFFFF    LLFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFvvvvFFFFll                                            
                          FFFFFFFFFFFFFFFFFFFFFF    ..FFFFFFFFFFFF  FFFFFFFFFFFFFFFF!!llllll!!ll                                            
                              FFFFFFFFFFFFFFFFFFFF                  FFFFFFFFFFFFFFFFllllllllllll                                            
                            !!    FFFFFFFFFFFFFFFF                  FFFFFFFFFFFFFFFFllllllllllllll                                          
                            vvLLLLll      ;;!!                      FFFFFFFFFFFFFFFFllllllllllll                                            
                            ;;LLLLLLLLLLLLLLLLvv                      FFFFFFFFFFFFFF!!llllll                                                
                              vvLLvvLLLLLLLLLLvv                            ..,,;;..      LLvv                                              
                              vvLLllLLLLLLLLLLvv                      ;;vvLLLLLLLLLLLLLLLLLL!!                                              
                                vvLLLLLLLLLLvv                          vvLLLLLLLLLLLLLLLLLL                                                
                                ;;llLLLLLLvv;;                          vvllLLLLLLLLLLLLLLvv                                                
                                  ;;vvLL!!vv                              !!LLLLLLLLLLLLLLvv                                                
                                    ;;vv!!                                ,,vvLLLLLLLLLL!!vv                                                
                                                                            ;;LLLLLLLLLLvv                                                  
                                                                            ;;vvLLLLLLvv;;                                                  
                                                                                ;;!!,,                                                      
        
```
