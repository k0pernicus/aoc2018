import os
import system
from part1 import solvePart1
from part2 import solvePart2

proc main() =
  if paramCount() == 0:
    echo "Please to add the file as program argument"
    quit(QuitFailure)
  let filePath = paramStr(1)
  if not existsFile(filePath):
    echo "Unfortunately, this file does not exists in your system"
    quit(QuitFailure)
  echo "Solution for part_1 is ", solvePart1(filePath)
  echo "Solution for part_2 is ", solvePart2(filePath)
when isMainModule:
  main()
