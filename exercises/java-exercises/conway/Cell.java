package cellularAutomata.conway;
import cellularAutomata.utils.*;
import javafx.scene.paint.Color;
import cellularAutomata.utils.ModularPoint;
import java.util.*;

public class Cell implements IState<Color>{
	private ModularPoint pt;
	private	boolean isAlive;
	private Cells field;
	public Cell(int r,int c,int d, boolean v){
		this(r,c,d);
		isAlive=v;
	}
	public Cell(int r, int c, int d){
		pt = new ModularPoint(r,c,d,d);
	}
	public void addTo(Cells c){
		field = c;
	}
	public void kill(){
		isAlive = false;
	}
	public void live(){
		isAlive = true;
	}
	public boolean life(){
		return isAlive;
	}
	public ModularPoint getPt(){
		return pt;
	}
	public int getNeighbors(){
		int o = 0;
		for (int i = -1; i <= 1; i++){
			for (int j = -1; j <= 1; j++){
				if (i == 0 && j == 0){
				}else{
					o += field.getCellMap()[pt.immutTrans(i,j).getPos(0).getValue()][getPt().immutTrans(i,j).getPos(1).getValue()].life()?1:0;
				}
			}
		}
		return o;
	}
/*	{
		int o = 0;
		for (int i = pt.getPos(0)-1; i < pt.getPos(0)+1; i++){
			for (int j = pt.getPos(0)-1; j < pt.getPos(1)+1; j++){
				o+=neighborhood.contains(new Point(i,j))?1:0;
			}
		}
		return o;
	}*/
	public Color getCurrentState(){
		return life()?Color.CYAN:Color.BROWN;
	}
	public Color getNextState(){
		if (getNeighbors()==2&&life()){
			return Color.CYAN;
		}else if (getNeighbors()==3){
			return Color.CYAN;
		}else{
			return Color.BROWN;
		}
	}
}
