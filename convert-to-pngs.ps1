Get-ChildItem generated | ForEach-Object {
  $file = $_
  $outName = "generated/" + $file.Name + ".png"
  C:\Program` Files\Inkscape\bin\inkscape.exe --export-type png --export-filename $outName -w 1920 $file.FullName
  while ($true) {
    $inkscape = Get-Process inkscape -ErrorAction SilentlyContinue
    if ($null -eq $inkscape) {
      break
    }
    Start-Sleep 1
  }
}