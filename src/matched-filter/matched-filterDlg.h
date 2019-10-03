// matched-filterDlg.h : header file
//

#pragma once

#include <util/common/gui/SimulationDialog.h>
#include <util/common/gui/PlotControl.h>

#include <libmf.hpp>

#include "model.h"

// CMatchedFilterDlg dialog
class CMatchedFilterDlg : public CSimulationDialog
{
    // Construction
public:
    CMatchedFilterDlg(CWnd* pParent = nullptr);    // standard constructor

// Dialog Data
#ifdef AFX_DESIGN_TIME
    enum { IDD = IDD_MATCHEDFILTER_DIALOG };
#endif

protected:
    virtual void DoDataExchange(CDataExchange* pDX);    // DDX/DDV support

// Implementation
protected:
    HICON m_hIcon;

    model::model_data m_data;
    model::demo_results m_demoResults;

    CPlotControl m_cIPlot;
    CPlotControl m_cQPlot;
    CPlotControl m_cFPlot;
    CPlotControl m_cResultsPlot;

    // Generated message map functions
    virtual BOOL OnInitDialog();
    afx_msg void OnPaint();
    afx_msg HCURSOR OnQueryDragIcon();
    DECLARE_MESSAGE_MAP()
public:
    afx_msg void OnBnClickedButton3();
    libmf::ffi::Params MakeParams();
    void FillPlot(const libmf::ffi::Signal& s, model::points_t& pts);
};
