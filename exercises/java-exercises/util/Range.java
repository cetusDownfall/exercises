package cellularAutomata.utils;
public class Range{
	private int upper, lower;
	public Range(int u, int l){
		if (u > l){
			upper = u;
			lower = l;
		} else {
			upper = l;
			lower = u;
		}
	}
	public boolean contains(int n){
		return (n < upper && n > lower);
	}
	public int getLower(){
		return lower;
	}
	public int getUpper(){
		return upper;
	}
	public int edge(int n){
		return n==(upper-1)?1:n==lower+1?-1:0;
	}
}
