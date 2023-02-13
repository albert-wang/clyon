#include <stdint.h>
#pragma once
#ifndef CYLON_INCLUDED
#define CYLON_INCLUDED
extern "C"
{
	struct LyonPathBuilder;
	struct LyonPath;
	struct LyonGeometry16;
	struct LyonGeometry32;

	struct LyonVector
	{
		float x;
		float y;
	};

	struct LyonPoint
	{
		float x;
		float y;
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

		uint32_t color;
		float fillIndex;
		float shapeIndex;
		
		float tolerance;
	};

	inline LyonPoint LyonCreatePoint(float x, float y)
	{
		return LyonPoint{ x, y };
	}

	inline LyonVector LyonCreateVector(float x, float y)
	{
		return LyonVector{ x, y };
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

	/// Creates an empty path builder.
	LyonPathBuilder*	LyonCreatePathBuilder				();

	/// Starts a new sub-path at the given position. 
	/// Roughly correlates to the SVG command "M".
	/// 
	/// This command creates a new initial and current point, as if the 
	/// pen was lifted and moved to a new location. If an existing sub-path was
	/// in progress, it is ended without being closed.
	void				LyonPathBuilder_MoveTo				(LyonPathBuilder*, LyonPoint);

	/// Closes the sub-path by connecting it to its starting point
	/// with a straight line.
	/// Roughly correlates to the SVG command "Z"
	///
	/// This ends the sub-path.
	void                LyonPathBuilder_Close 				(LyonPathBuilder*);	
	void				LyonPathBuilder_LineTo				(LyonPathBuilder*, LyonPoint);
	void				LyonPathBuilder_QuadraticBeizerTo	(LyonPathBuilder*, LyonPoint ctrl, LyonPoint ending);
	void				LyonPathBuilder_SmoothQuadraticBeizerTo(LyonPathBuilder*, LyonPoint ending);
	void				LyonPathBuilder_CubicBeizerTo		(LyonPathBuilder*, LyonPoint ctrl1, LyonPoint ctrl2, LyonPoint end);
	void				LyonPathBuilder_SmoothCubicBeizerTo (LyonPathBuilder*, LyonPoint ctrl2, LyonPoint end);
	void				LyonPathBuilder_Arc					(LyonPathBuilder*, LyonPoint center, float rX, float rY, float sweepRadians, float xRotation);
	void				LyonPathBuilder_ArcTo				(LyonPathBuilder*, LyonPoint to, float rX, float rY, float xRotation, int32_t large, int32_t sweep);
	void				LyonPathBuilder_HorizontalLineTo    (LyonPathBuilder*, float x);
	void				LyonPathBuilder_VerticalLineTo    	(LyonPathBuilder*, float y);


	void 				LyonPathBuilder_RelativeMoveTo      (LyonPathBuilder*, LyonVector);
	void 				LyonPathBuilder_RelativeLineTo      (LyonPathBuilder*, LyonVector);
	void				LyonPathBuilder_RelativeQuadraticBeizerTo		(LyonPathBuilder*, LyonVector ctrl, LyonVector end);
	void				LyonPathBuilder_RelativeSmoothQuadraticBeizerTo	(LyonPathBuilder*, LyonVector ending);
	void				LyonPathBuilder_RelativeCubicBeizerTo       	(LyonPathBuilder*, LyonVector ctrl, LyonVector ctrl2, LyonVector end);
	void				LyonPathBuilder_RelativeSmoothCubicBeizerTo 	(LyonPathBuilder*, LyonVector ctrl2, LyonVector end);
	void				LyonPathBuilder_RelativeArcTo		(LyonPathBuilder*, LyonPoint to, float rX, float rY, float xRotation, int32_t large, int32_t sweep);

	void				LyonPathBuilder_Reserve				(LyonPathBuilder*, uint64_t Endpoints, uint64_t ControlPoints);

	LyonPoint			LyonPathBuilder_GetCurrentPosition  		(LyonPathBuilder*);

	// These do not have a relative variant.
	void 				LyonPathBuilder_AddRect(LyonPathBuilder*, LyonPoint min, LyonPoint max);
	void 				LyonPathBuilder_AddCircle(LyonPathBuilder*, LyonPoint center, float radius);
	void 				LyonPathBuilder_AddRoundedRect(LyonPathBuilder*, LyonPoint min, LyonPoint max, float radius);
	void				LyonPathBuilder_AddEllipse(LyonPathBuilder*, LyonPoint center, float rX, float rY, float xRotation);

	// This function 'consumes' the PathBuilder, and frees it.
	// Any additional access to the LyonPathBuilder after this function is invalid.
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

	uint32_t				LyonVersion();
}
#endif