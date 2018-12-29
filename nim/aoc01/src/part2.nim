import intsets
import sequtils
import strutils

from itertools import cycle

type numbers = seq[int]

proc parse(filePath: string): numbers =
  result = readFile(filePath).strip().splitLines().map(parseInt)

proc getFrequency(l: numbers): int =
  var frequency = 0
  var seen = initIntSet()

  for f in l.cycle():
    frequency += f
    if frequency notin seen:
      seen.incl(frequency)
    else:
      break

  result = frequency

proc solvePart2*(filePath: string): int =
  result = getFrequency(parse(filePath))
