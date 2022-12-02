
$get = Get-Content /home/matt/advent_of_code_2022/files/day1.txt
foreach($g in $get){
    if($g.Length -eq 0){
        $elf += @($dump)
        $dump = 0
    } else {
        $dump += [int]$g
    }
    $i++
    if($i -gt ($get.Count -1)){
        $elf += @($dump)
    }
}
$greedest_elf = $elf.IndexOf([int]($elf | Measure-Object -Maximum).Maximum)
$most = ($elf | Measure-Object -Maximum).Maximum
Write-Host "Elf number $($greedest_elf+1) with $($most) calories"
