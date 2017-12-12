package cellularAutomata.utils;
import javafx.scene.paint.Color;

public class CursorPoint extends Point{
	private int[] v = new int[]{0,0,0,0}; //ldur
	private ModularInteger speed;
	private RangeFinder bounds;
	private Color c = Color.LIME;
	private boolean slow = false;
	private boolean moved = false;
	public CursorPoint(int r, int c){
		super(r/2,c/2);
		bounds = new RangeFinder(new Point(-1,-1),new Point(r,c));
		speed = new ModularInteger(0,720/r);
	}
	public int getSpeed(){
		return speed.getValue();
	}
	public void setInstSpeed(int i){
		speed.set(i);
	}
	public void setSlow(){
		slow = true;
	}
	public void setFast(){
		slow = false;
	}
	public boolean getSlow(){
		return slow;
	}
	public boolean motion(){
		return moved;
	}
	public void stop(){
		for (int n : v){
			n = 0;
		}
	}
	public void move(){
		int[] netV = new int[2];
		netV[0] = v[1]-v[2];
		netV[1] = v[3]-v[0];
		boolean canMoveR = true;
		boolean canMoveC = true;
		if (bounds.contains(this)&&speed.getValue()==0){
			switch(bounds.get(0).edge(getPos(0))){
				case -1: canMoveR = netV[0] > 0;break;
				case 1: canMoveR = netV[0] < 0;break;
				case 0: canMoveR = true;break;
			}
			switch(bounds.get(1).edge(getPos(1))){
				case -1: canMoveC = netV[1] > 0;break;
				case 1: canMoveC = netV[1] < 0;break;
				case 0: canMoveC = true;break;
			}
			translate(0,canMoveR?netV[0]:0);
			translate(1,canMoveC?netV[1]:0);
		}
		moved = !(netV[0] == 0 && netV[1] == 0);
		if (!moved && slow){
			speed.set(0);
		}else if (moved && slow){
			speed.set(1);
		}else {
			speed.add(1);
		}
	}
	public void push(char d, int m){
		switch (d){
			case 'l': v[0] = m; break;
			case 'd': v[1] = m; break;
			case 'u': v[2] = m; break;
			case 'r': v[3] = m; break;
		}
	}
	public Color getColor(){
		return c;
	}
	public void setColor(Color olor){
		c = olor;
	}
}
