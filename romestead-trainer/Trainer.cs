using System;

namespace RomesteadTrainer
{
    public class Trainer
    {
        private int experience = 0;
        private int level = 1;

        public void StartSession()
        {
            Console.WriteLine("Training session started.");
            Console.WriteLine("Press any key to gain experience...");

            while (true)
            {
                var key = Console.ReadKey(true);
                if (key.Key == ConsoleKey.Escape)
                    break;

                GainExperience(10);
                Console.WriteLine($"Level: {level} | XP: {experience}");
            }

            Console.WriteLine("Session ended.");
        }

        public void GainExperience(int xp)
        {
            experience += xp;
            if (experience >= level * 100)
            {
                level++;
                experience = 0;
                Console.WriteLine("Level up!");
            }
        }

        public int GetLevel() => level;
        public int GetExperience() => experience;
    }
}