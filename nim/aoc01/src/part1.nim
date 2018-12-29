import math
import sequtils
import strutils

const filePath = "input.txt"

type lines = seq[string]

proc parse(filePath: string): lines =
  result = readFile(filePath).strip().splitLines()

proc getSum(l: lines): int =
  result = l.map(parseInt).sum

proc solvePart1*(filePath: string): int = 
  result = getSum(parse(filePath))
