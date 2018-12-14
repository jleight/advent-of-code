using System;
using System.Linq;
using System.Threading.Tasks;
using AoC.Abstractions;

namespace AoC.Y2017.Solutions
{
    public class D02P2 : SolutionBase
    {
        public override Task Run()
        {
            var input = InputLines
                .Select(l => l.Split("\t").Select(n => int.Parse(n)).ToArray())
                .ToArray();

            var sum = 0;
            foreach (var line in input)
            {
                for (var a = 0; a < input.Length; a++)
                for (var b = 0; b < input.Length; b++)
                {
                    if (a == b || line[a] % line[b] > 0)
                        continue;
                    sum += line[a] / line[b];
                }
            }

            Console.WriteLine(sum);
            return Task.CompletedTask;
        }
    }
}
