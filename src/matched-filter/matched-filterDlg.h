// matched-filterDlg.h : header file
//

#pragma once

#include <util/common/gui/SimulationDialog.h>

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

    // Generated message map functions
    virtual BOOL OnInitDialog();
    afx_msg void OnPaint();
    afx_msg HCURSOR OnQueryDragIcon();
    DECLARE_MESSAGE_MAP()
};
