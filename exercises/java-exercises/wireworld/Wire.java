package cellularAutomata.wireworld;

import javafx.scene.paint.Color;
import cellularAutomata.utils.Point;
import java.util.ArrayList;
import java.util.*;
import cellularAutomata.utils.IState;
import java.util.Set;
import java.util.HashSet;
import cellularAutomata.utils.*;

public class Wire implements IState<Color>{
	private Wire[][] field;
	private HashSet<Wire> wires = new HashSet<Wire>();
	private ModularInteger clock = new ModularInteger(0,5);
	private boolean isClock = false;
	
	public enum CellType{
		DEAD (Color.BLACK),
		WIRE (Color.ORANGE.brighter()),
		HEAD   (Color.WHITE),
		TAIL  (Color.CYAN);
		private Color color;
		private CellType(Color color){
			this.color = color;
		}
		public Color getCurrentState(){
			return color;
		}
	};
	CellType type = CellType.DEAD;
	CellType nextType = getCellType();
	Point pt = new Point(0,0);
	public Point getPoint(){
		return pt;
	}
	public boolean isNeighbor(Wire w){
		int distC = Math.abs(getPoint().getPos(1) - w.getPoint().getPos(1));
		int distR = Math.abs(getPoint().getPos(0) - w.getPoint().getPos(0));
		return (distR <= 1 && distC <= 1) && !(distR == 0 && distC == 0);
	}
	public void update(){
		type = nextType;
		clock.add(1);
	}
	@Override
	public Color getNextState(){
		switch (type){
			case DEAD: nextType = CellType.DEAD; break;
			case WIRE: nextType = (getHeadCt()==1||getHeadCt()==2)?CellType.HEAD:CellType.WIRE;break;
			case HEAD: nextType = CellType.TAIL; break;
			case TAIL: nextType = CellType.WIRE; break;
			default: nextType = CellType.DEAD; break;
		}
		if (isClock && toPoint().getPos(1) == 0 && !getCurrentState().equals(CellType.DEAD.getCurrentState())){
			if (clock.getValue()==0){
				nextType = CellType.HEAD;
			}
			if (clock.getValue()==1){
				nextType = CellType.TAIL;
			}
		}
		return nextType.getCurrentState();
	}
	@Override
	public Color getCurrentState(){
		return getCellType().getCurrentState();
	}
	public Wire(int r, int c){
		type = CellType.DEAD;
		pt = new Point(r,c);
		wires.add(this);
	}
	public void setClock(boolean v){
		isClock = v;
	}
	public Wire(int r, int c, CellType t){
		type = t;
		getNextState();
		pt = new Point(r,c);
		wires.add(this);
	}
	public CellType getCellType(){
		return type;
	}
	public void setCellType(CellType c){
		type = c;
	}
	public int getHeadCt(){
		int o = 0;
		for (Wire l : getWires()){
			o += ((isNeighbor(l)&&l.getCellType().getCurrentState().equals(CellType.HEAD.getCurrentState())))?1:0;
		}
		return o;
	}
	public void addTo(Wire[][] f, HashSet<Wire> h){
		field = f;
		wires = h;
		field[getPoint().getPos(0)][getPoint().getPos(1)] = this;
		if (getCellType().ordinal() != 0){
			wires.add(this);
		}
	}
	public HashSet<Wire> getWires(){
		return wires;
	}
	public Point toPoint(){
		return pt;
	}
}
