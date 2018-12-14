using System;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace AoC.Abstractions
{
    public abstract class SolutionBase : ISolution
    {
        private string _input;
        private string[] _inputLines;


        protected Stream Input { get; }
        protected string InputString => _input ?? (_input = GetInputString());
        protected string[] InputLines => _inputLines ?? (_inputLines = GetInputLines());


        public SolutionBase()
        {
            var name = GetType().Name;
            Input = GetInput(name) ?? GetInput(name.Split("P").FirstOrDefault());
        }


        public abstract Task Run();

        private Stream GetInput(string name)
        {
            var assembly = GetType().Assembly;

            var resource = assembly
                .GetManifestResourceNames()
                .FirstOrDefault(r => r.Contains(name));
            if (resource is null)
                return null;

            return assembly
                .GetManifestResourceStream(resource);
        }

        private string GetInputString()
        {
            if (Input is null)
                return null;

            return new StreamReader(Input)
                .ReadToEnd();
        }

        private string[] GetInputLines()
        {
            if (InputString is null)
                return null;

            return InputString
                .Split(new[] { '\r', '\n' }, StringSplitOptions.RemoveEmptyEntries);
        }
    }
}
