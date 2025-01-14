// this needs to implement stuff
enum WasmToHost{
     End_1,
     Log_1{},
     CompileWebGLShader_1{},
     AllocArrayBuffer_1{},
     AllocIndexBuffer_1{},
     AllocVao_1{},
     DrawCall_1{},
     Clear_1{},
     LoadDeps_1{},
     UpdateTextureImage2D_1{},
     RequestAnimationFrame_1{},
     SetDocumentTitle_1{},
     SetMouseCursor_1{},
     ReadFile_1{},
     ShowTextIME_1{},
     HideTextIME_1{},
     TextCopyResponse_1{},
     StartTimer_1{},
     StopTimer_1{},
     XRStartPresenting_1{},
     XRStopPresenting_1{},
     BeginRenderTargets_1{},
     AddColorTarget_1{},
     SetDepthTarget_1{},
     EndRenderTargets_1{},
     SetDefaultDepthAndBlendMode_1{},
     BeginMainCanvas_1{},
     HTTPSend_1{},
     WebSocketSend_1{},
     FullScreen_1{},
     NormalScreen_1{},
 }
 
 enum HostToWasm{
     GetJSApi_1{},
     End_1,
     InitDeps_1{},
     DepsLoaded_1{},
     Init_1{},
     Resize_1{},
     AnimationFrame_1{},
     FingerDown_1{},
     FingerUp_1{},
     FingerMove_1{},
     FingerHover_1{},
     FingerScroll_1{},
     FingerOut_1{},
     KeyDown_1{},
     KeyUp_1{},
     TextInput_1{},
     FileRead_1{},
     FileError_1{},
     TextCopy_1{},
     Timer_1{},
     FocusLost_1{},
     XRUpdate_1{},
     PaintDirty_1{},
     HTTPSendResponse_1{},
     WebSocketMessage_1{},
     WebSocketError_1{}
 }
