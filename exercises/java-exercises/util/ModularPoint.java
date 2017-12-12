package cellularAutomata.utils;

public class ModularPoint {
	private ModularInteger rowPos;
	private ModularInteger colPos;
	private int cRange, rRange;
	public ModularPoint(int r, int c, int rd, int cd){
		rowPos = new ModularInteger(r,rd);
		colPos = new ModularInteger(c,cd);
		rowPos.refresh();
		colPos.refresh();
		rRange = rowPos.getUpper();
		cRange = colPos.getUpper();
	}
	public int getRange(int i){
		if (i == 0) {
			return rRange;
		}else if (i == 1) {
			return cRange;
		}else{
			return 0;
		}
	}
	public void translate(int rd, int cd){
		getPos(0).add(rd);
		getPos(1).add(cd);
	}
	public ModularInteger getPos(int i){
		if (i == 0){
			return rowPos;
		}else if (i == 1){
			return colPos;
		}else{
			return new ModularInteger(0,getRange(0));
		}
	}
	public Point toPoint(){
		return new Point(getPos(0).getValue(),getPos(1).getValue());
	}
	public ModularPoint immutTrans(int rd, int cd){
		return new ModularPoint(getPos(0).immutAdd(rd).getValue(),getPos(1).immutAdd(cd).getValue(),getRange(0),getRange(1));
	}
}
