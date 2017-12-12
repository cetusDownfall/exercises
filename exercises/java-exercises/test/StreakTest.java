import java.util.Random;
import java.util.ArrayList;
import java.util.List;
public class StreakTest{
	private static class Streak{
		private int length = 1;
		private boolean type;
		public Streak(boolean typ){
			type = typ;
		}
		public String toString(){
			return new String("A streak of " + (type?"heads":"tails") + " flips of " + length + " length just ended.");
		}
		public void increment(){
			length++;
		}
		public boolean getType(){
			return type;
		}
	}
	public static void main(String[] args){
		Random gen = new Random();
		int tries = Integer.parseInt(args[0]);
		ArrayList<Streak> streaks = new ArrayList<Streak>();
		for(int i = 0; i < tries; i++){
			boolean tr = ((int)(gen.nextFloat()+.5))==1;
			if(i==0){
				Streak tmp = new Streak(tr);
				streaks.add(tmp);
			}else{
				if(!(streaks.get(streaks.size()-1).getType()^tr)){
					streaks.get(streaks.size()-1).increment();
				}else{
					System.out.println(streaks.get(streaks.size()-1));
				}
			}
		}
	}
}
