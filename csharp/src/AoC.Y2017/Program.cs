using System.Linq;
using System.Threading.Tasks;

namespace AoC.Y2017
{
    internal static class Program
    {
        static Task Main(string[] args)
            => Solution.Run(typeof(Program), args?.FirstOrDefault());
    }
}
