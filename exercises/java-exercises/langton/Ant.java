package cellularAutomata.langton;
import cellularAutomata.utils.*;

public class Ant extends Point{
	private int dir = 0;
	public Ant(int x, int y, int d){
		super(x,y);
		dir = d;
	}
	public Ant(Point p, int d){
		super(p.getPos(0),p.getPos(1));
		dir = d;
	}
	public int getDir(){
		return dir;
	}
	public void turn(boolean c){
		if(c){
			turnRight();
		}else{
			turnLeft();
		}
	}
	public void turnRight(){
		if (dir == 3)
			dir = -1;
		dir++;
	}
	public void turnLeft(){
		if (dir == 0)
			dir = 4;
		dir--;
	}
	public void move(boolean c){
		turn(c);
		switch (dir){
			case 0: translate(1, -1); break;
			case 1: translate(0, 1); break;
			case 2: translate(1, 1); break;
			case 3: translate(0, -1); break;
		}
	}
	public int getToroidalPos(int i, int dim){
		int outPos;
		outPos = getPos(i)%dim;
		outPos = outPos+dim;
		outPos = outPos%dim;
		return outPos;
	}
	public void toroidalMove(boolean c, int dim){
		toroidalMove(c, dim, dim);
	}
	public boolean equals(Ant a){
		return getPos(0) == a.getPos(0) && getPos(1) == a.getPos(1) && getDir() == a.getDir();
	}
	public void toroidalMove(boolean c, int dimr, int dimc){
		move(c);
		setPos(1, getToroidalPos(1, dimr));
		setPos(0, getToroidalPos(0, dimc));
	}
}
