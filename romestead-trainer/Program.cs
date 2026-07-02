using System;

namespace RomesteadTrainer
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Romestead Trainer v1.0");
            Console.WriteLine("Initializing training environment...");

            var trainer = new Trainer();
            trainer.StartSession();
        }
    }
}