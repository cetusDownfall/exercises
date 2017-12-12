package cellularAutomata.wireworld;
import cellularAutomata.utils.CursorPoint;
import cellularAutomata.wireworld.Wire.CellType;
public class WireDrawPoint extends CursorPoint{
	CellType t = CellType.DEAD;
	boolean drw = false;
	public WireDrawPoint(int r, int c){
		super(r,c);
	}
	public void setType(CellType teep){
		t = teep;
	}
	public CellType getType(){
		return t;
	}
	public void setDraw(boolean v){
		drw = v;
	}
	public boolean getDraw(){
		return drw;
	}
}
