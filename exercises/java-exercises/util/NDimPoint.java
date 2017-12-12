package cellularAutomata.utils;
/**	Can hold a position of n dimensions.
  *	Ryugo Okada
  *	12/01/2017
  */
public abstract class NDimPoint{
	private int[] pos;
	public NDimPoint(int n){
		this(n,new int[n]);
	}
	public NDimPoint(int n, int[] ords){
		pos = new int[n];
		for(int i = 0; i < ords.length; i++){
			pos[i] = ords[i];
		}
	}
	public void setPos(int n, int v){
		pos[n] = v;
	}
	public void set(int... ords){
		if(these.length == pos.length){
			for(int i = 0; i < ords.length; i++){
				pos[i]=ords[i];
			}
		}
		System.err.println("Arg length of " + this + " was invalid.");
	}
	public int getDegree(){
		return pos.length;
	}
	public int getPos(int n){
		return pos[n];
	}
	public int[] get(){
		return pos;
	}
}
