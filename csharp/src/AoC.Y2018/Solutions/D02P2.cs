using System;
using System.Linq;
using System.Threading.Tasks;
using AoC.Abstractions;

namespace AoC.Y2018.Solutions
{
    public class D02P2 : SolutionBase
    {
        public override Task Run()
        {
            for (var ai = 0; ai < InputLines.Length; ai++)
            {
                var a = InputLines[ai];

                for (var bi = ai + 1; bi < InputLines.Length; bi++)
                {
                    var b = InputLines[bi];

                    var diffs = 0;
                    for (var i = 0; i < a.Length; i++)
                    {
                        diffs += a[i] == b[i] ? 0 : 1;
                        if (diffs > 1)
                            continue;
                    }
                    if (diffs != 1)
                        continue;

                    for (var i = 0; i < a.Length; i++)
                    {
                        if (a[i] == b[i])
                            Console.Write(a[i]);
                    }
                    Console.WriteLine();
                    return Task.CompletedTask;
                }
            }

            Console.WriteLine("No off-by-ones!");
            return Task.CompletedTask;
        }
    }
}
