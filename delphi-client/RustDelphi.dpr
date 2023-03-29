program RustDelphi;

{$APPTYPE CONSOLE}

{$R *.res}

uses
  System.SysUtils, Winapi.Windows;

var
  x, y: Float32;

function add(x, y: Float32): Float32; stdcall; external 'calculations.dll';

begin
  try
    Readln(x);
    Readln(y);
    Writeln(FloatToStr(add(x, y)));
    Sleep(5000);
  except
    on E: Exception do
      begin
        Writeln(E.ClassName, ': ', E.Message);
        Sleep(5000);
      end;
  end;
end.
