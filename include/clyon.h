#include <stdint.h>

extern "C"
{
	struct LyonPathBuilder;
	struct LyonPath;
	struct LyonGeometry16;
	struct LyonGeometry32;

	struct LyonInputVertex
	{
		float position[2];
	};

	enum LyonPrimitiveType
	{
		LyonPrimitiveText = 0,
		LyonPrimitiveFilled = 1,
		LyonPrimitiveStroked = 2
	};

	struct LyonOutputVertex
	{
		float position[2];
		float uv[2];
		float normal[2];
		uint32_t color;

		float primitiveType;
		float fillIndex;
		float shapeIndex;
	};

	struct LyonAABB
	{
		float lowerLeft[2];
		float upperRight[2];
	};

	enum LyonFillRule
	{
		LyonFillRuleEvenOdd = 0,
		LyonFillRuleNonZero = 1
	};

	enum LyonOrientation
	{
		LyonOrientationHorizontal = 0,
		LyonOrientationVertical = 1
	};

	enum LyonLineCap
	{
		LyonLineCapButt = 0,
		LyonLineCapSquare = 1,
		LyonLineCapRound = 2
	};

	enum LyonLineJoin
	{
		LyonLineJoinMiter = 0,
		LyonLineJoinMiterClip = 1,
		LyonLineJoinRound = 2,
		LyonLineJoinBevel = 3
	};

	struct LyonFillProperties
	{
		float tolerance;
		int32_t fillRule;
		int32_t orientation;

		uint32_t color;
		float fillIndex;
		float shapeIndex;
	};

	struct LyonStrokeProperties
	{
		int32_t startCap;
		int32_t endCap;
		int32_t join;
		float width;
		int32_t applyWidth;

		uint32_t color;
		float fillIndex;
		float shapeIndex;
	};

	inline LyonInputVertex LyonCreatePoint(float x, float y)
	{
		LyonInputVertex v = { 0 };
		v.position[0] = x;
		v.position[1] = y;

		return v;
	}

	inline LyonFillProperties LyonCreateFillProperties()
	{
		LyonFillProperties props = { 0 };
		return props;
	}

	inline LyonFillProperties LyonCreateFillPropertiesWithColorAndIndices(uint32_t color, float fill, float shape)
	{
		LyonFillProperties props = { 0 };
		props.color = color;
		props.fillIndex = fill;
		props.shapeIndex = shape;
		return props;
	}

	inline LyonStrokeProperties	LyonCreateStrokeProperties()
	{
		LyonStrokeProperties props = { 0 };
		return props;
	}

	inline LyonStrokeProperties	LyonCreateStrokePropertiesWithColorAndIndices(uint32_t color, float fill, float shape)
	{
		LyonStrokeProperties props = { 0 };
		props.color = color;
		props.fillIndex = fill;
		props.shapeIndex = shape;
		return props;
	}

	// LyonPathBuilder functions
	LyonPathBuilder*	LyonCreatePathBuilder				();
	void                LyonPathBuilder_Begin               (LyonPathBuilder*, LyonInputVertex);
	void                LyonPathBuilder_End                 (LyonPathBuilder*, bool close);

	void				LyonPathBuilder_LineTo				(LyonPathBuilder*, LyonInputVertex);
	void				LyonPathBuilder_QuadraticBeizerTo	(LyonPathBuilder*, float ctrlX, float ctrlY, LyonInputVertex);
	void				LyonPathBuilder_CubicBeizerTo		(LyonPathBuilder*, float ctrlX, float ctrlY, float ctrl2X, float ctrl2Y, LyonInputVertex);
	void				LyonPathBuilder_Arc					(LyonPathBuilder*, LyonInputVertex center, float rX, float rY, float startRadians, float sweepRadians, float xRotation);
	void				LyonPathBuilder_ArcTo				(LyonPathBuilder*, LyonInputVertex to, float rX, float rY, float xRotation, int32_t large, int32_t sweep);
	LyonPath*			LyonPathBuilder_Build				(LyonPathBuilder*);

	// LyonPath functions
	LyonAABB			LyonPathBoundingRect	(LyonPath*);
	void				LyonFreePath			(LyonPath*);
	LyonGeometry16*		LyonTessellateFill16	(LyonPath*, LyonFillProperties);
	LyonGeometry16*		LyonTessellateStroke16	(LyonPath*, LyonStrokeProperties);
	LyonGeometry32*		LyonTessellateFill32	(LyonPath*, LyonFillProperties);
	LyonGeometry32*		LyonTessellateStroke32	(LyonPath*, LyonStrokeProperties);

	const LyonOutputVertex* LyonGeometry16_VerticesData		(LyonGeometry16*);
	const uint16_t*			LyonGeometry16_IndicesData		(LyonGeometry16*);
	uint32_t				LyonGeometry16_VerticesLength	(LyonGeometry16*);
	uint32_t				LyonGeometry16_IndicesLength	(LyonGeometry16*);
	void					LyonFreeGeometry16				(LyonGeometry16*);

	const LyonOutputVertex* LyonGeometry32_VerticesData		(LyonGeometry32*);
	const uint16_t*			LyonGeometry32_IndicesData		(LyonGeometry32*);
	uint32_t				LyonGeometry32_VerticesLength	(LyonGeometry32*);
	uint32_t				LyonGeometry32_IndicesLength	(LyonGeometry32*);
	void					LyonFreeGeometry32				(LyonGeometry32*);

}