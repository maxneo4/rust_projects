#NoEnv  ; Recommended for performance and compatibility with future AutoHotkey releases.
; #Warn  ; Enable warnings to assist with detecting common errors.
SendMode Input  ; Recommended for new scripts due to its superior speed and reliability.
SetWorkingDir %A_ScriptDir%  ; Ensures a consistent starting directory.
;shift=+

^!1::
Run send_file_as_keys.exe 01_createInputClass.txt 25, , Hide
return

^!2::
Run send_file_as_keys.exe 02_addUsings.txt 25, , Hide
return

^!3::
Run send_file_as_keys.exe 03_createScenariosInstance.txt 75, , Hide
return

^!4::
Run send_file_as_keys.exe 04_implRunCode.txt 25, , Hide
return

^!5::
Run send_file_as_keys.exe 05_implValidateCode.txt 150, , Hide
return

^!6::
Run send_file_as_keys.exe 06_writeCaseWithOneReferenced.txt 75, , Hide
return

^!x::
Process, Close, send_file_as_keys.exe
return

^!n::
Run send_file_as_keys.exe yaml.txt 75, , Hide
return

